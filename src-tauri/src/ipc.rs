// Inter-Process Communication between Rust and SvelteKit

use crate::custom_types::{Error, DateLimit}; // Import the Error type
use crate::database::establish_connection;
use crate::database::search::{search_fts_index, get_recently_opened_docs};
use crate::database::models::SearchResult;
use diesel::SqliteConnection;
use tauri_plugin_shell; // Import the tauri_plugin_shell crate
use crate::indexing::{walk_directory, all_allowed_filetypes};
use crate::housekeeping;
use tokio::sync::mpsc;

// Get allowed filetypes
#[tauri::command]
fn get_allowed_filetypes() -> Result<Vec<String>, Error> {
  let allowed_filetypes = all_allowed_filetypes();
  // Convert ALLOWED_FILETYPES to Vec<String>
  Ok(allowed_filetypes.iter().map(|s| s.to_string()).collect())
}

// Open a file (in default app) or a folder from the path
#[tauri::command]
fn open_file_or_folder(file_path: String, window: tauri::Window) -> Result<String, Error> {
  println!("Window {} invoked this command to open {}", window.label(), file_path);
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

// Run the file watcher script
#[tauri::command]
async fn run_file_indexing() -> Result<String, Error> {
  println!("File watcher started");
  let home_directory = housekeeping::get_home_directory().unwrap();
  println!("Home directory: {}", home_directory);

  let (sender, mut receiver) = mpsc::channel::<usize>(1);

  tokio::spawn(async move {
    // let files_added = walk_directory(&home_directory);
    let files_added = walk_directory(&home_directory);
    sender.send(files_added).await.expect("Failed to send data through channel");
  });

  // Receive the result from the channel asynchronously
  if let Some(files_added) = receiver.recv().await {
    println!("Files added: {}", files_added);
  }
  Ok("File indexing complete".to_string())
}

// Run search
#[tauri::command]
fn run_search(
  query: String,
  page: i32,
  limit: i32,
  file_type: Option<String>,
  date_limit: Option<DateLimit>
) -> Result<Vec<SearchResult>, Error> {
  println!("run_search: query: {}, page: {}, limit: {}, file_type: {:?}, date_limit: {:?}", query, page, limit, file_type, date_limit);
  let conn: SqliteConnection = establish_connection();
  let search_results = search_fts_index(query, page, limit, file_type, date_limit, conn).unwrap();
  Ok(search_results)
}

// Get recently opened documents
#[tauri::command]
fn get_recent_docs(
  page: i32,
  limit: i32,
  file_type: Option<String>
) -> Result<Vec<SearchResult>, Error> {
  let conn: SqliteConnection = establish_connection();
  let search_results = get_recently_opened_docs(page, limit, file_type, conn).unwrap();
  Ok(search_results)
}

// Open QuickLook (MacOS) or Peek (Windows)
#[tauri::command]
fn open_quicklook(file_path: String) -> Result<String, Error> {
  println!("Opening QuickLook for {}", file_path);

  #[cfg(target_os = "macos")]
  let _ = std::process::Command::new("qlmanage")
    .arg("-p")
    .arg(file_path)
    .output();
  #[cfg(windows)]
  let _ = std::process::Command::new("cmd")
    .arg("/C")
    .arg("start")
    .arg("powershell")
    .arg("peek")
    .arg(file_path)
    .output();
  Ok("Opened QuickLook!".into())
}

// Add global shortcut to hide or show the window
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
use tauri::Manager;
use crate::window::hide_or_show_window;

fn get_global_shortcut(modifier: Modifiers, key: Code) -> Shortcut {
  // TODO: Modify this so it pulls values from user settings instead of args
  Shortcut::new(Some(modifier), key)
}

pub fn initialize() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_allowed_filetypes,
      open_file_or_folder,
      open_folder_containing_file,
      run_file_indexing,
      run_search,
      get_recent_docs,
      open_quicklook
    ])
    .plugin(tauri_plugin_shell::init())
      .setup(|app| {
        #[cfg(desktop)]
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
          app.global_shortcut().register(get_global_shortcut(Modifiers::ALT, Code::Space))?;
        }
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}