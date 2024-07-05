// use crate::custom_types::Error; // Import the Error type
use dirs::document_dir;
use crate::database::create_tables_if_not_exists;
use crate::database::establish_direct_connection_to_db;
use crate::utils::norm;
use crate::user_prefs::{set_default_app_data, set_default_user_prefs, set_default_file_types};
use crate::custom_types::UserPreferencesState;
use log::{info, LevelFilter};
use jfs::Store;

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

pub fn setup_logging_file_path() {
  let documents_dir = get_documents_directory().unwrap();
  let app_dir_path = format!("{}/{}", documents_dir, APP_DIRECTORY);
  let logging_file_path = format!("{}/{}", app_dir_path, "buzee.log");
  let logging_file_path = norm(&logging_file_path);
  println!("Logging to file: {}", &logging_file_path);
  let _ = simple_logging::log_to_file(logging_file_path, LevelFilter::Info);
  info!("Logger initialized!");
}

pub fn get_user_preferences_store_path() -> String {
  let documents_dir = get_documents_directory().unwrap();
  let app_dir_path = format!("{}/{}", documents_dir, APP_DIRECTORY);
  let user_preferences_store_path = format!("{}/{}", app_dir_path, "buzee-user-preferences.json");
  let user_preferences_store_path = norm(&user_preferences_store_path);
  
  user_preferences_store_path
}

pub fn setup_user_preferences_store_path() {
  let user_preferences_store_path = get_user_preferences_store_path();
  println!("User preferences stored in file: {}", &user_preferences_store_path);

  let mut cfg = jfs::Config::default();
  cfg.single = true;
  let user_preferences_store = jfs::Store::new_with_cfg(user_preferences_store_path, cfg).unwrap();
  let default_user_preferences = UserPreferencesState { 
    first_launch_done: false,
    onboarding_done: false,
    show_search_suggestions: true,
    launch_at_startup: true,
    show_in_dock: true,
    global_shortcut_enabled: true,
    global_shortcut: "Alt+Space".to_string(),
    automatic_background_sync: true,
    detailed_scan: true
  };
  let id = user_preferences_store.save(&default_user_preferences).unwrap();
  let obj = user_preferences_store.get::<UserPreferencesState>(&id).unwrap();
  println!("User preferences store initialized with defaults: {:?}", obj);
  // user_preferences_store.delete(&id).unwrap();
  info!("User preferences store initialized with defaults!");
}

// Initialisation function called on each app load
pub fn initialize() -> () {
  println!("Initializing app directory");
  create_app_directory_if_not_exists().unwrap();
  
  // Set up logging
  setup_logging_file_path();

  // Set up user preferences store
  setup_user_preferences_store_path();

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