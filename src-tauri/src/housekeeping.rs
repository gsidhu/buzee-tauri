// use crate::custom_types::Error; // Import the Error type
use dirs::document_dir;
use crate::database::create_tables_if_not_exists;
use crate::database::establish_direct_connection_to_db;
use crate::utils::norm;
use crate::user_prefs::{set_default_app_data, set_default_user_prefs, set_default_file_types};
use log::{info, LevelFilter};

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
  let app_dir_path = norm(&app_dir_path);
  println!("creating app dir at:{}", &app_dir_path);
  std::fs::create_dir_all(app_dir_path)
}

pub fn get_app_directory() -> String {
  let app_dir_path = format!("{}/{}", get_documents_directory().unwrap(), APP_DIRECTORY);
  let app_dir_path = norm(&app_dir_path);
  app_dir_path
}

pub fn create_tantivy_index_directory_if_not_exists() -> Result<(), std::io::Error> {
  let app_dir_path = get_app_directory();
  let index_dir_path = format!("{}/{}", app_dir_path, "buzee_tantivy_index");
  let index_dir_path = norm(&index_dir_path);
  println!("creating tantivy index dir at:{}", &index_dir_path);
  std::fs::create_dir_all(index_dir_path)
}

pub fn setup_logging_file_path() {
  let documents_dir = get_documents_directory().unwrap();
  let app_dir_path = format!("{}/{}", documents_dir, APP_DIRECTORY);
  let logging_file_path = format!("{}/{}", app_dir_path, "buzee.log");
  let logging_file_path = norm(&logging_file_path);
  println!("Logging to file: {}", &logging_file_path);
  let _ = simple_logging::log_to_file(logging_file_path, LevelFilter::Info);
  info!("Logger initialized!");
}

// Initialisation function called on each app load
pub fn initialize() -> () {
  println!("Initializing app directory");
  create_app_directory_if_not_exists().unwrap();
  create_tantivy_index_directory_if_not_exists().unwrap();
  
  // Set up logging
  setup_logging_file_path();

  // Set up user preferences store
  // setup_user_preferences_store_path();

  let mut conn = establish_direct_connection_to_db();
  println!("Initializing database");
  create_tables_if_not_exists(&mut conn).unwrap();

  // Set default app data
  set_default_app_data(&mut conn);
  // Set default user prefs
  set_default_user_prefs(&mut conn, false);
  // Set default file types
  set_default_file_types(&mut conn);
}