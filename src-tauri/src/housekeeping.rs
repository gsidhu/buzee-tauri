// use crate::custom_types::Error; // Import the Error type
use dirs::document_dir;
use crate::database::create_tables_if_not_exists;
use crate::database::establish_connection;
use crate::user_prefs::{set_default_app_data, set_default_user_prefs, set_default_file_types, set_scan_running_status};
use log::LevelFilter;

pub const APP_DIRECTORY: &str = r#"buzee-tauri"#;

// Get the documents directory
// MacOS: /Users/<username>/Documents
// Windows: C:\Users\<username>\My Documents
pub fn get_documents_directory() -> Option<String> {
  if let Some(documents_dir) = document_dir() {
    Some(documents_dir.to_string_lossy().to_string())
  } else {
    None
  }
}

pub fn get_home_directory() -> Option<String> {
  if let Some(home_dir) = dirs::home_dir() {
    Some(home_dir.to_string_lossy().to_string())
  } else {
    None
  }
}

// Create the app directory in the documents directory
// In case it doesn't exist
pub fn create_app_directory_if_not_exists() -> Result<(), std::io::Error> {
  let documents_dir = get_documents_directory().unwrap();
  let app_dir_path = format!("{}/{}", documents_dir, APP_DIRECTORY);
  std::fs::create_dir_all(app_dir_path)
}

pub fn setup_logging_file_path() {
  let documents_dir = get_documents_directory().unwrap();
  let app_dir_path = format!("{}/{}", documents_dir, APP_DIRECTORY);
  let logging_file_path = format!("{}/{}", app_dir_path, "buzee.log");
  println!("Logging to file: {}", &logging_file_path);
  let _ = simple_logging::log_to_file(logging_file_path, LevelFilter::Info);
}

// Initialisation function called on each app load
pub fn initialize() -> () {
  // Set up logging
  setup_logging_file_path();
  println!("Initializing app directory");
  create_app_directory_if_not_exists().unwrap();
  println!("Initializing database");
  create_tables_if_not_exists(establish_connection()).unwrap();

  // Set default app data
  set_default_app_data();
  // Set default user prefs
  set_default_user_prefs();
  // Set default file types
  set_default_file_types();

  // On each launch, set the sync_running flag to false
  set_scan_running_status(false, false);
}