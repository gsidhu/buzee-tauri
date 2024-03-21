use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::database::establish_connection;
use crate::database::models::{AppData, UserPrefs, FileTypes};
use crate::database::schema::{app_data, user_preferences, file_types};
use crate::housekeeping::get_home_directory;
use crate::ipc::send_message_to_frontend;
use crate::indexing::{walk_directory, parse_content_from_files, remove_nonexistent_files};
use log::{error, info, trace, warn};

pub fn run_sync_operation(window: tauri::Window) -> (usize, usize) {
  info!("Starting sync operation");
  // Emit starting sync status to the frontend
  send_message_to_frontend(&window, "sync-status".to_string(), "sync-status".to_string(), "true".to_string());
  let home_directory = get_home_directory().unwrap();
  // Parse metadata of all files but only update the ones whose time metadata or size has changed
  let files_added = walk_directory(&window, &home_directory);
	// Then run through all the files and remove the ones that no longer exist
  remove_nonexistent_files();
	// Then start parsing the content of all files and add it to the body table
	// let files_parsed = parse_content_from_files();
  let files_parsed = 0;
  // Emit closing sync status to the frontend
  send_message_to_frontend(&window, "sync-status".to_string(), "sync-status".to_string(), "false".to_string());
  (files_added, files_parsed)
}

pub fn sync_status() -> String {
  let mut conn = establish_connection();

  let sync_running: Vec<bool> = app_data::table
    .select(app_data::scan_running)
    .load::<bool>(&mut conn)
    .expect("Error loading app_data");
  
  if sync_running[0] {
    "true".to_string()
  } else {
    "false".to_string()
  }
}