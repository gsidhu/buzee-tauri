// Inter-Process Communication between Rust and SvelteKit

use crate::error_type::Error; // Import the Error type
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
  walk_directory(&home_directory);
  Ok("File watcher started".into())
}

pub fn initialize() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      open_file_folder,
      run_file_watcher_script
    ])
    .plugin(tauri_plugin_shell::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}