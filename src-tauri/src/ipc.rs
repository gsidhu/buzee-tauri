// Inter-Process Communication between Rust and SvelteKit
// The idea is to eventually keep only callers here and move the actual logic to other files. This way, for creating a web app, we just have to convert this file into an API and call the same functions from the frontend.

use crate::custom_types::{DBStat, DateLimit, Error, Payload, GlobalShortcutState};
use crate::database::establish_connection;
use crate::database::models::DocumentSearchResult;
use crate::database::search::{
    get_counts_for_all_filetypes, get_recently_opened_docs, search_fts_index, get_metadata_title_matches,
};
use crate::db_sync::{run_sync_operation, sync_status};
use crate::housekeeping;
use crate::indexing::{all_allowed_filetypes, walk_directory};
use crate::user_prefs::{set_new_global_shortcut_in_db, set_global_shortcut_from_db, get_global_shortcut, get_modifiers_and_code_from_global_shortcut};
use crate::window::hide_or_show_window;
use serde_json;
use tauri::Manager;
use tauri_plugin_shell;
use tokio::sync::mpsc;
use tauri::menu::Menu;
use crate::context_menu::{contextmenu_receiver, searchresult_context_menu, statusbar_context_menu,};
use log::{error, info, trace, warn};
use schedule::{Agenda, Job};
use std::sync::{Arc, Mutex};
use std::process::Command;
use tauri_plugin_shell::ShellExt;

pub fn send_message_to_frontend(
    window: &tauri::WebviewWindow,
    event: String,
    message: String,
    data: String,
) {
  window.emit(&event, Payload { message, data }).unwrap();
}

fn setup_cron_job(window: tauri::WebviewWindow) {
  let mut sched = Agenda::new();
  let window_clone = Arc::new(Mutex::new(window));
  sched.add(Job::new(move || {
    info!("Running background sync cron job");
    // convert Arc<std::sync::Mutex<tauri::Window>> to tauri::Window
    let window_clone = window_clone.lock().unwrap().clone();
    // run_sync_operation(window_clone);
  }, "0 * * * * *".parse().unwrap()));

  // Check and run pending jobs in agenda every 60 seconds
  loop {
    sched.run_pending();
    std::thread::sleep(std::time::Duration::from_millis(10000));
  }
}

// Get OS boolean
#[tauri::command]
fn get_os() -> Result<String, Error> {
    let os = std::env::consts::OS;
    Ok(os.to_string())
}

// Get allowed filetypes
#[tauri::command]
fn get_allowed_filetypes() -> Result<String, Error> {
  let mut connection = establish_connection();
  let allowed_filetypes = all_allowed_filetypes(&mut connection, true);
  // Convert allowed_filetypes to JSON using serde_json
  let json_response = serde_json::to_string(&allowed_filetypes).unwrap();
  Ok(json_response)
}

// Open a file (in default app) or a folder from the path
#[tauri::command]
fn open_file_or_folder(file_path: String, window: tauri::Window) -> Result<String, Error> {
    println!(
        "Window {} invoked this command to open {}",
        window.label(),
        file_path
    );
    let _ = open::that(file_path);
    // If it worked
    Ok("Opened the file or folder!".into())
}

// Open the folder containing the file from the filepath
#[tauri::command]
fn open_folder_containing_file(file_path: String) -> Result<String, Error> {
    println!("Opening folder for {}", file_path);
    let do_steps = || -> Result<(), Error> {
        #[cfg(target_os = "windows")]
        {
          // This opens the folder in the background, which is not desirable
          // Maybe remove it in future
          Command::new("explorer")
          .args(["/select,", &file_path]) // The comma after select is not a typo
          .spawn()
          .map_err(|e| e)?;
        }

        #[cfg(target_os = "macos")]
        {
          let path_buf = std::path::PathBuf::from(&file_path);
          if path_buf.is_dir() {
          Command::new("open")
            .args([&file_path])
            .spawn()
            .map_err(|e| e)?;
          } else {
          Command::new("open")
            .args(["-R", &file_path])
            .spawn()
            .map_err(|e| e)?;
          }
        }
        Ok(())
    };

    if let Err(_err) = do_steps() {
        let path = std::path::PathBuf::from(file_path);
        let dir = path.parent().unwrap();
        let _ = open::that_detached(dir);
    }
    Ok("Opened the folder!".into())
}

// Run file indexing ONLY
#[tauri::command]
async fn run_file_indexing(window: tauri::WebviewWindow) -> Result<String, Error> {
  let mut connection = establish_connection();
  println!("File watcher started");
  let home_directory = housekeeping::get_home_directory().unwrap();
  println!("Home directory: {}", home_directory);

  let (sender, mut receiver) = mpsc::channel::<usize>(1);

  tokio::spawn(async move {
      let files_added = walk_directory(&mut connection, &window, &home_directory);
      sender
          .send(files_added)
          .await
          .expect("Failed to send data through channel");
  });

  // Receive the result from the channel asynchronously
  if let Some(files_added) = receiver.recv().await {
      println!("Files added: {}", files_added);
  }
  Ok("File indexing complete".to_string())
}

// Run file sync
#[tauri::command]
async fn run_file_sync(app: tauri::AppHandle, window: tauri::WebviewWindow) {
  run_sync_operation(window, app).await;
}

// Get sync status
#[tauri::command]
fn get_sync_status() -> Result<String, Error> {
  let mut conn = establish_connection();
  let sync_running = sync_status(&mut conn);
  Ok(sync_running)
}

// Get search suggestions
#[tauri::command]
fn get_search_suggestions(query: String) -> Result<Vec<String>, Error> {
  let mut conn = establish_connection();
  let suggestions = get_metadata_title_matches(query, &mut conn).unwrap();
  Ok(suggestions)
}

// Run search
#[tauri::command]
fn run_search(
    query: String,
    page: i32,
    limit: i32,
    file_type: Option<String>,
    date_limit: Option<DateLimit>,
) -> Result<Vec<DocumentSearchResult>, Error> {
    println!(
        "run_search: query: {}, page: {}, limit: {}, file_type: {:?}, date_limit: {:?}",
        query, page, limit, file_type, date_limit
    );
    let conn = establish_connection();
    let search_results = search_fts_index(query, page, limit, file_type, date_limit, conn).unwrap();
    Ok(search_results)
}

// Get recently opened documents
#[tauri::command]
fn get_recent_docs(
    page: i32,
    limit: i32,
    file_type: Option<String>,
) -> Result<Vec<DocumentSearchResult>, Error> {
    let conn = establish_connection();
    let search_results = get_recently_opened_docs(page, limit, file_type, conn).unwrap();
    Ok(search_results)
}

// Get DB Stats
#[tauri::command]
fn get_db_stats() -> Result<Vec<DBStat>, Error> {
    let conn = establish_connection();
    let db_stats = get_counts_for_all_filetypes(conn).unwrap();
    Ok(db_stats)
}

// Open QuickLook (MacOS) or Peek (Windows)
#[tauri::command]
fn open_quicklook(file_path: String) -> Result<String, Error> {
    println!("Opening QuickLook for {}", file_path);

    #[cfg(target_os = "macos")]
    std::thread::spawn(move || {
        let _ = std::process::Command::new("qlmanage")
            .arg("-p")
            .arg(file_path)
            .output();
    });

    #[cfg(target_os = "windows")]
    std::thread::spawn(move || {
      let home_directory = home_dir().unwrap().to_string_lossy().to_string();
      let quicklook_path = format!("{}\\AppData\\Local\\Programs\\QuickLook\\QuickLook.exe", &home_directory);
      let _ = std::process::Command::new(quicklook_path)
        .arg(file_path)
        .output();
    });
    Ok("Opened QuickLook!".into())
}

// Context Menu
#[tauri::command]
fn open_context_menu(window: tauri::Window, option: String) {
    match option.as_str() {
        "searchresult" => searchresult_context_menu(&window),
        "statusbar" => statusbar_context_menu(&window),
        _ => println!("Invalid context menu option"),
    }
}

// Set new global shortcut in DB and update the global shortcut
#[tauri::command]
async fn set_new_global_shortcut(app_handle: tauri::AppHandle, new_shortcut_string: String) {
  println!("Setting new global shortcut: {}", new_shortcut_string);
  set_new_global_shortcut_in_db(new_shortcut_string);
  set_global_shortcut_from_db(&app_handle);
  // restart the app to set the new shortcut
  app_handle.restart();
}

#[tauri::command]
async fn enable_sidecar(app: tauri::AppHandle) {
  #[cfg(target_os = "macos")] 
  {
    let sidecar_command = app.shell().sidecar("textra").unwrap().args(["/Users/thatgurjot/Desktop/Gurjot_Arc.png", "-o", "/Users/thatgurjot/Desktop/arc.txt"]);
    let (mut _rx, mut _child) = sidecar_command.spawn().unwrap();
  }
}

pub fn initialize() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_allowed_filetypes,
      get_os,
      open_file_or_folder,
      open_folder_containing_file,
      run_file_indexing,
      run_file_sync,
      get_sync_status,
      get_search_suggestions,
      run_search,
      get_recent_docs,
      get_db_stats,
      open_quicklook,
      open_context_menu,
      set_new_global_shortcut,
      enable_sidecar
    ])
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
        #[cfg(desktop)]
        {
          // manage state(s)
          let handle = app.handle();
          handle.manage(Mutex::new(GlobalShortcutState::default()));
        }
        {
          set_global_shortcut_from_db(app.handle());
          app.handle().plugin(
            tauri_plugin_global_shortcut::Builder::new()
              .with_shortcut(get_global_shortcut(app.handle()))?
              .with_handler(|app_handle, shortcut| {
                  let (global_shortcut_modifiers, global_shortcut_code) = get_modifiers_and_code_from_global_shortcut(app_handle);
                  if shortcut.matches(global_shortcut_modifiers, global_shortcut_code) {
                    println!("Global Shortcut Detected!");
                    let main_window = app_handle.get_webview_window("main").unwrap();
                    hide_or_show_window(main_window);
                  }
              })
              .build(),
          )?;
        }
        {
          app.on_menu_event(|app_handle: &tauri::AppHandle, event| {
            println!("menu event: {:?}", event);
            // let main_window = app_handle.get_webview_window("main").unwrap();
            contextmenu_receiver(app_handle, event);
          });
        }
        {
          // let main_window = app.get_webview_window("main").unwrap();
          // setup_cron_job(main_window);
        }
        Ok(())
    })
    .menu(|app_handle| Menu::default(app_handle))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
