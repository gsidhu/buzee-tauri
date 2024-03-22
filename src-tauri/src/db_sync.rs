use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};
use crate::database::establish_connection;
use crate::database::schema::app_data;
use crate::housekeeping::get_home_directory;
use crate::ipc::send_message_to_frontend;
use crate::indexing::{walk_directory, parse_content_from_files, remove_nonexistent_files};
use log::{error, info, trace, warn};

pub fn run_sync_operation(window: tauri::Window) {
  info!("Starting sync operation");
  let mut connection = establish_connection();
  // Emit starting sync status to the frontend
  send_message_to_frontend(&window, "sync-status".to_string(), "sync-status".to_string(), "true".to_string());
  let home_directory = get_home_directory().unwrap();
  // Parse metadata of all files but only update the ones whose time metadata or size has changed
  let _files_added = walk_directory(&mut connection, &window, &home_directory);
	// Then run through all the files and remove the ones that no longer exist
  remove_nonexistent_files(&mut connection);
	// Then start parsing the content of all files and add it to the body table
	let _files_parsed = parse_content_from_files(&mut connection);
  // let files_parsed = 0;
  // Emit closing sync status to the frontend
  println!("Sending message to frontend: Sync operation completed");
  send_message_to_frontend(&window, "sync-status".to_string(), "sync-status".to_string(), "false".to_string());
}

pub fn sync_status(conn: &mut SqliteConnection) -> String {
  let sync_running: Vec<bool> = app_data::table
    .select(app_data::scan_running)
    .load::<bool>(conn)
    .expect("Error loading app_data");
  
  if sync_running[0] {
    "true".to_string()
  } else {
    "false".to_string()
  }
}