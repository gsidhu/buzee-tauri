// Inter-Process Communication between Rust and SvelteKit

use crate::custom_types::Error; // Import the Error type
use crate::database::establish_connection;
use crate::database::search::search_fts_index;
use crate::database::models::SearchResult;
use diesel::SqliteConnection;
use tauri_plugin_shell; // Import the tauri_plugin_shell crate
use crate::indexing::walk_directory;
use crate::housekeeping;

// Open the folder from the filepath
#[tauri::command]
fn open_file_folder(file_path: String, window: tauri::Window) -> Result<String, Error> {
  println!("Window {} invoked this command to open {}", window.label(), file_path);
  let _ = open::that(file_path);
  // If it worked
  Ok("This worked!".into())
}

// Run the file watcher script
#[tauri::command]
fn run_file_watcher_script() -> Result<String, Error> {
  println!("File watcher started");
  let home_directory = housekeeping::get_home_directory().unwrap();
  println!("Home directory: {}", home_directory);
  std::thread::spawn(move || {
    walk_directory(&home_directory);
  });
  Ok("File watcher started".into())
}

// Run search
#[tauri::command]
fn run_search(
  query: String,
  page: i32,
  limit: i32,
  file_type: Option<String>,
) -> Result<Vec<SearchResult>, Error> {
  println!("run_search: query: {}, page: {}, limit: {}, filetype: {:?}", query, page, limit, file_type);
  let conn: SqliteConnection = establish_connection();
  let search_results = search_fts_index(query, page, limit, file_type, conn).unwrap();
  Ok(search_results)
}

pub fn initialize() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      open_file_folder,
      run_file_watcher_script,
      run_search
    ])
    .plugin(tauri_plugin_shell::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}