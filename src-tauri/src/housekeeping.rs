// use crate::error_type::Error; // Import the Error type
use dirs::document_dir;
use crate::schema::create_tables_if_not_exists;
use crate::database::establish_connection;
// use std::path::PathBuf;

pub const APP_DIRECTORY: &str = r#"buzee-tauri"#;

pub fn get_documents_directory() -> Option<String> {
  if let Some(documents_dir) = document_dir() {
    Some(documents_dir.to_string_lossy().to_string())
  } else {
    None
  }
}

pub fn create_app_directory_if_not_exists() -> Result<(), std::io::Error> {
  let documents_dir = get_documents_directory().unwrap();
  let app_dir_path = format!("{}/{}", documents_dir, APP_DIRECTORY);
  std::fs::create_dir_all(app_dir_path)
}

pub fn initialize() -> () {
  println!("Initializing app directory");
  create_app_directory_if_not_exists().unwrap();
  println!("Initializing database");
  create_tables_if_not_exists(establish_connection()).unwrap();
}