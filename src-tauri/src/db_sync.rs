use diesel::SqliteConnection;
// use diesel::{QueryDsl, RunQueryDsl};
use tauri::{AppHandle, Manager};
use crate::database::establish_connection;
// use crate::database::schema::app_data;
use crate::housekeeping::get_home_directory;
use crate::ipc::send_message_to_frontend;
use crate::indexing::{walk_directory, parse_content_from_files};
use crate::user_prefs::set_scan_running_status;
use log::{error, info, trace, warn};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::custom_types::SyncRunningState;

pub async fn run_sync_operation(window: tauri::WebviewWindow, app: AppHandle) {
  println!("File sync started");
  let mut conn = establish_connection(&app);

  // On each click, check if sync is already running
  let sync_running = sync_status(&app);
  if sync_running.0 == "true" {
    println!("File sync already running; Stopping now");
    // Set sync status to false
    set_scan_running_status(&mut conn, false, true, &app);
    println!("File sync stopped");
  } else {
    info!("FILE SYNC STARTED AT {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
    // Set sync status to true
    set_scan_running_status(&mut conn, true, true, &app);
    // Spawn the new task
    tokio::spawn(async move {
      // Emit starting sync status to the frontend
      send_message_to_frontend(&window, "sync-status".to_string(), "sync-status".to_string(), "true".to_string());
      let home_directory = get_home_directory().unwrap();
      // Parse metadata of all files but only update the ones whose time metadata or size has changed
      let _files_added = walk_directory(&mut conn, &window, &home_directory);
      // Then start parsing the content of all files and add it to the body table
      println!("Parsing content from files");
      let files_parsed = parse_content_from_files(&mut conn, app.clone()).await;
      println!("Files parsed: {}", files_parsed);
      // let files_parsed = 0;
      // Emit closing sync status to the frontend
      println!("Sending message to frontend: Sync operation completed");
      send_message_to_frontend(&window, "sync-status".to_string(), "sync-status".to_string(), "false".to_string());
      set_scan_running_status(&mut conn, false, true, &app);
      info!("FILE SYNC FINISHED AT {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
    });
  }
}

pub fn sync_status(app: &AppHandle) -> (String, i64) {
  let state_mutex = app.state::<Mutex<SyncRunningState>>();
  let state = state_mutex.lock().unwrap();
  let sync_running = &state.sync_running;
  let last_sync_time = &state.last_sync_time;
  
  if sync_running == &true {
    ("true".to_string(), *last_sync_time)
  } else {
    ("false".to_string(), *last_sync_time)
  }
}