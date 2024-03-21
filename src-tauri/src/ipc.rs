// Inter-Process Communication between Rust and SvelteKit

use crate::custom_types::{DBStat, DateLimit, Error, Payload, ThreadManager};
use crate::database::establish_connection;
use crate::database::models::DocumentSearchResult;
use crate::database::search::{
    get_counts_for_all_filetypes, get_recently_opened_docs, search_fts_index,
};
use crate::db_sync::{run_sync_operation, sync_status};
use crate::housekeeping;
use crate::indexing::{all_allowed_filetypes, walk_directory};
use crate::user_prefs::set_scan_running_status;
use crate::window::hide_or_show_window;
use diesel::SqliteConnection;
use lazy_static::lazy_static;
use log::{error, info, trace, warn};
use serde_json;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};
use tauri_plugin_shell; // Import the tauri_plugin_shell crate
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle, Toplevel};

lazy_static::lazy_static! {
    static ref THREAD_MANAGER: Arc<Mutex<ThreadManager>> = Arc::new(Mutex::new(ThreadManager::new()));
}

// App Menu
use tauri::menu::Menu;
// Import context menu commands
#[cfg(desktop)]
use crate::context_menu::{
    contextmenu_receiver, searchresult_context_menu, statusbar_context_menu,
};

pub fn send_message_to_frontend(
    window: &tauri::Window,
    event: String,
    message: String,
    data: String,
) {
    window.emit(&event, Payload { message, data }).unwrap();
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
    let allowed_filetypes = all_allowed_filetypes(true);
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
    println!("Opening file folder for {}", file_path);
    let path = std::path::PathBuf::from(file_path);
    let dir = path.parent().unwrap();
    let _ = open::that(dir);
    Ok("Opened the folder!".into())
}

// Run file indexing ONLY
#[tauri::command]
async fn run_file_indexing(window: tauri::Window) -> Result<String, Error> {
    println!("File watcher started");
    let home_directory = housekeeping::get_home_directory().unwrap();
    println!("Home directory: {}", home_directory);

    let (sender, mut receiver) = mpsc::channel::<usize>(1);

    tokio::spawn(async move {
        let files_added = walk_directory(&window, &home_directory);
        // let files_added = walk_directory("/Users/thatgurjot/Desktop/");
        // let files_added = parse_content_from_files();
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
async fn run_file_sync(window: tauri::Window) -> Result<String, Error> {
    println!("File sync started");

    // Acquire the lock on the thread manager
    let mut thread_manager = THREAD_MANAGER.lock().await;

    // Check if the task is already running
    if let Some(handle) = thread_manager.handle.take() {
      println!("File sync already running; Stopping now");
      // If it's running, stop the task
      handle.abort();
      // Await the completion of the previous task
      if let Err(_) = handle.await {
          println!("File sync task aborted");
      }
    } else {
      // Spawn the new task
      let (sender, mut receiver) = mpsc::channel::<(usize, usize)>(1);
      let handle: JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> =
          tokio::spawn(async move {
              let (files_added, files_parsed) = run_sync_operation(window);
              sender
                  .send((files_added, files_parsed))
                  .await
                  .expect("Failed to send data through channel");
              Ok(())
          });

      // Store the handle in the thread manager
      thread_manager.handle = Some(handle);
    
      // Receive the result from the channel asynchronously
      if let Some((files_added, files_indexed)) = receiver.recv().await {
        println!("Files added: {}", files_added);
        println!("Files indexed: {}", files_indexed);
      }
    }
    // Release the lock
    drop(thread_manager);

    set_scan_running_status(false, true);
    Ok("File indexing complete".to_string())
}

// Get sync status
#[tauri::command]
fn get_sync_status() -> Result<String, Error> {
    let sync_running = sync_status();
    Ok(sync_running)
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
    let conn: SqliteConnection = establish_connection();
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
    let conn: SqliteConnection = establish_connection();
    let search_results = get_recently_opened_docs(page, limit, file_type, conn).unwrap();
    Ok(search_results)
}

// Get DB Stats
#[tauri::command]
fn get_db_stats() -> Result<Vec<DBStat>, Error> {
    let conn: SqliteConnection = establish_connection();
    let db_stats = get_counts_for_all_filetypes(conn).unwrap();
    Ok(db_stats)
}

// Open QuickLook (MacOS) or Peek (Windows)
#[tauri::command]
fn open_quicklook(file_path: String) -> Result<String, Error> {
    println!("Opening QuickLook for {}", file_path);

    #[cfg(target_os = "macos")]
    // spawn a new thread to open QuickLook using `std` crate
    std::thread::spawn(move || {
        let _ = std::process::Command::new("qlmanage")
            .arg("-p")
            .arg(file_path)
            .output();
    });

    #[cfg(target_os = "windows")]
    std::thread::spawn(move || {
        let _ = std::process::Command::new("cmd")
            .arg("/C")
            .arg("start")
            .arg("powershell")
            .arg("peek")
            .arg(file_path)
            .output();
    });
    Ok("Opened QuickLook!".into())
}

// Add global shortcut to hide or show the window
fn get_global_shortcut(modifier: Modifiers, key: Code) -> Shortcut {
    // TODO: Modify this so it pulls values from user settings instead of args
    Shortcut::new(Some(modifier), key)
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
            run_search,
            get_recent_docs,
            get_db_stats,
            open_quicklook,
            open_context_menu
        ])
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            {
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::with_handler(|_app, shortcut| {
                        let global_shortcut = get_global_shortcut(Modifiers::ALT, Code::Space);
                        println!("{:?}", shortcut);
                        if shortcut == &global_shortcut {
                            println!("Alt+Space Detected!");
                            let main_window = _app.get_webview_window("main").unwrap();
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
            Ok(())
        })
        .menu(|app_handle| Menu::default(app_handle))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
