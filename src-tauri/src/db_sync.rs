use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};
use tauri::AppHandle;
use crate::database::establish_connection;
use crate::database::schema::app_data;
use crate::housekeeping::get_home_directory;
use crate::ipc::send_message_to_frontend;
use crate::indexing::{walk_directory, parse_content_from_files};
use crate::user_prefs::set_scan_running_status;
use log::{error, info, trace, warn};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run_sync_operation(window: tauri::WebviewWindow, app: AppHandle) {
  info!("FILE SYNC STARTED AT {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
  println!("File sync started");
  let mut conn = establish_connection();

  // On each click, check if sync is already running
  let sync_running = sync_status(&mut conn);
  if sync_running == "true" {
    println!("File sync already running; Stopping now");
    // Set sync status to false
    set_scan_running_status(&mut conn, false, true);
    println!("File sync stopped");
  } else {
    // Set sync status to true
    set_scan_running_status(&mut conn, true, true);
    // Spawn the new task
    tokio::spawn(async move {
      // Emit starting sync status to the frontend
      send_message_to_frontend(&window, "sync-status".to_string(), "sync-status".to_string(), "true".to_string());
      let home_directory = get_home_directory().unwrap();
      // Parse metadata of all files but only update the ones whose time metadata or size has changed
      let _files_added = walk_directory(&mut conn, &window, &home_directory);
      // Then start parsing the content of all files and add it to the body table
      println!("Parsing content from files");
      let _files_parsed = parse_content_from_files(&mut conn, app.clone());
      // let files_parsed = 0;
      // Emit closing sync status to the frontend
      println!("Sending message to frontend: Sync operation completed");
      send_message_to_frontend(&window, "sync-status".to_string(), "sync-status".to_string(), "false".to_string());
      set_scan_running_status(&mut conn, false, true);
      info!("FILE SYNC FINISHED AT {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
    });
  }
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