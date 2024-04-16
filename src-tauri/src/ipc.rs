// Inter-Process Communication between Rust and SvelteKit
// The idea is to eventually keep only callers here and move the actual logic to other files. This way, for creating a web app, we just have to convert this file into an API and call the same functions from the frontend.

use crate::custom_types::{DBStat, DateLimit, Error, Payload, DBConnPoolState, GlobalShortcutState, SyncRunningState};
use crate::database::{establish_connection, get_connection_pool};
use crate::database::models::DocumentSearchResult;
use crate::database::search::{
    get_counts_for_all_filetypes, get_recently_opened_docs, search_fts_index, get_metadata_title_matches,
};
use crate::db_sync::{run_sync_operation, sync_status};
use crate::housekeeping;
use crate::indexing::{all_allowed_filetypes, walk_directory};
use crate::user_prefs::{get_global_shortcut, get_modifiers_and_code_from_global_shortcut, is_global_shortcut_enabled, set_global_shortcut_state_from_db_value, set_new_global_shortcut_in_db, set_global_shortcut_flag_in_db};
use crate::window::hide_or_show_window;
use serde_json;
use tauri::Manager;
use tauri_plugin_shell;
use tokio::{sync::mpsc, time::{interval, Duration}};
use tauri::menu::Menu;
use crate::context_menu::{contextmenu_receiver, searchresult_context_menu, statusbar_context_menu,};
// use log::info;
use std::sync::Mutex;
use std::process::Command;

pub fn send_message_to_frontend(
    window: &tauri::WebviewWindow,
    event: String,
    message: String,
    data: String,
) {
  window.emit(&event, Payload { message, data }).unwrap();
}

// Setup cron job for background sync
#[tauri::command]
async fn setup_cron_job(window: tauri::WebviewWindow, app: tauri::AppHandle) {
  tokio::spawn(async move {
    let mut interval = interval(Duration::from_millis(1800000));
    interval.tick().await;
    loop {
      interval.tick().await;
      let sync_running = sync_status(&app);
      println!("??? Sync running: {}", sync_running.0);
      let current_timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
      if sync_running.0 == "false" && current_timestamp - sync_running.1 > 1800 {
        let window_clone = window.clone();
        let app_clone = app.clone();
        run_sync_operation(window_clone, app_clone).await;
      }
    }
  });
}

// Get OS boolean
#[tauri::command]
fn get_os() -> Result<String, Error> {
    let os = std::env::consts::OS;
    Ok(os.to_string())
}

// Get allowed filetypes
#[tauri::command]
fn get_allowed_filetypes(app: tauri::AppHandle) -> Result<String, Error> {
  let mut connection = establish_connection(&app);
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
async fn run_file_indexing(window: tauri::WebviewWindow, app: tauri::AppHandle) -> Result<String, Error> {
  let mut connection = establish_connection(&app);
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
fn get_sync_status(app: tauri::AppHandle) -> Result<String, Error> {
  let sync_running = sync_status(&app).0;
  Ok(sync_running)
}

// Get search suggestions
#[tauri::command]
fn get_search_suggestions(query: String, app: tauri::AppHandle) -> Result<Vec<String>, Error> {
  let mut conn = establish_connection(&app);
  let suggestions = get_metadata_title_matches(query, &mut conn).unwrap_or(vec![]);
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
    app: tauri::AppHandle
) -> Result<Vec<DocumentSearchResult>, Error> {
    println!(
        "run_search: query: {}, page: {}, limit: {}, file_type: {:?}, date_limit: {:?}",
        query, page, limit, file_type, date_limit
    );
    let conn = establish_connection(&app);
    let search_results = search_fts_index(query, page, limit, file_type, date_limit, conn).unwrap_or(vec![]);
    Ok(search_results)
}

// Get recently opened documents
#[tauri::command]
fn get_recent_docs(
    page: i32,
    limit: i32,
    file_type: Option<String>,
    app: tauri::AppHandle
) -> Result<Vec<DocumentSearchResult>, Error> {
    let conn = establish_connection(&app);
    let search_results = get_recently_opened_docs(page, limit, file_type, conn).unwrap();
    Ok(search_results)
}

// Get DB Stats
#[tauri::command]
fn get_db_stats(app: tauri::AppHandle) -> Result<Vec<DBStat>, Error> {
    let conn = establish_connection(&app);
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

// Toggle the global shortcut flag in DB
#[tauri::command]
async fn set_global_shortcut_flag(app_handle: tauri::AppHandle, flag: bool) {
  println!("Setting global shortcut flag to: {}", flag);
  set_global_shortcut_flag_in_db(flag, &app_handle);
  set_global_shortcut_state_from_db_value(&app_handle);
}
// Set new global shortcut in DB and update the global shortcut
#[tauri::command]
async fn set_new_global_shortcut(app_handle: tauri::AppHandle, new_shortcut_string: String) {
  println!("Setting new global shortcut: {}", new_shortcut_string);
  set_new_global_shortcut_in_db(new_shortcut_string, &app_handle);
  set_global_shortcut_state_from_db_value(&app_handle);
  // Tell user to restart the app for changes to take effect
  // restart the app to set the new shortcut
  // app_handle.restart();
}

pub fn initialize() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      setup_cron_job,
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
      set_global_shortcut_flag,
      set_new_global_shortcut,
      crate::drag::start_drag
    ])
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
        #[cfg(desktop)]
        {
          // manage state(s)
          let handle = app.handle();
          // db connection pool
          let pool = get_connection_pool();
          handle.manage(Mutex::new(DBConnPoolState::new(pool)));
          // global shortcut state
          handle.manage(Mutex::new(GlobalShortcutState::default()));
          // sync running state
          handle.manage(Mutex::new(SyncRunningState::default()));
        }
        {
          set_global_shortcut_state_from_db_value(app.handle());
          if is_global_shortcut_enabled(app.handle()) {
            println!("Global Shortcut is enabled");
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
          } else {
            println!("Global Shortcut is disabled");
          }
        }
        {
          app.on_menu_event(|app_handle: &tauri::AppHandle, event| {
            println!("menu event: {:?}", event);
            // let main_window = app_handle.get_webview_window("main").unwrap();
            contextmenu_receiver(app_handle, event);
          });
        }
        Ok(())
    })
    .menu(|app_handle| Menu::default(app_handle))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
