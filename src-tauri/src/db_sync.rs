// use diesel::{QueryDsl, RunQueryDsl};
use tauri::{AppHandle, Manager};
use crate::database::{establish_connection, establish_direct_connection_to_db};
// use crate::database::schema::app_data;
use crate::housekeeping::get_home_directory;
use crate::ipc::send_message_to_frontend;
use crate::indexing::{add_path_to_allow_list, parse_content_from_files, walk_directory};
use crate::user_prefs::set_scan_running_status;
use log::info;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::custom_types::{SyncRunningState, UserPreferencesState};

pub async fn run_sync_operation(window: tauri::WebviewWindow, app: AppHandle, switch_off: bool, file_paths: Vec<String>) {
  println!("file paths: {:?}", file_paths);
  let mut file_paths = file_paths;

  println!("File sync started");
  let mut conn = establish_connection(&app);

  // On each click, check if sync is already running
  let sync_running = sync_status(&app);
  if sync_running.0 == "true" || switch_off {
    println!("File sync already running; Stopping now");
    // Set sync status to false
    set_scan_running_status(&mut conn, false, true, &app);
    println!("File sync stopped");
  } else {
    info!("FILE SYNC STARTED AT {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
    // Set sync status to true
    set_scan_running_status(&mut conn, true, true, &app);

    let app_clone = app.clone();
    let state_mutex = app_clone.state::<Mutex<UserPreferencesState>>();
    let state = state_mutex.lock().unwrap();
    let detailed_scan_allowed = state.detailed_scan;
    // Spawn the new task
    tokio::spawn(async move {
      // Emit starting sync status to the frontend
      send_message_to_frontend(&window, "sync-status".to_string(), "sync-status".to_string(), "true".to_string());
      let home_directory = get_home_directory().unwrap_or("/".to_string());
      // Parse metadata of all files but only update the ones whose time metadata or size has changed
      if file_paths.len() == 0 {
        file_paths.push(home_directory.clone());
      }
      let mut new_conn = establish_connection(&app);
      let _files_added = walk_directory(&mut new_conn, &window, file_paths);

      if detailed_scan_allowed {
        // Then start parsing the content of all files and add it to the body table
        println!("Parsing content from files");
        let files_parsed = parse_content_from_files(&mut new_conn, app.clone()).await;
        println!("Files parsed: {}", files_parsed);
      }
      // Emit closing sync status to the frontend
      println!("Sending message to frontend: Sync operation completed");
      set_scan_running_status(&mut new_conn, false, true, &app);
      send_message_to_frontend(&window, "sync-status".to_string(), "sync-status".to_string(), "false".to_string());
      info!("FILE SYNC FINISHED AT {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
      send_message_to_frontend(&window, "file-sync-finished".to_string(), "sync-finished".to_string(), "true".to_string());
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

pub async fn add_specific_folders(window: &tauri::WebviewWindow, app: &AppHandle, file_paths: Vec<String>, is_folder: bool) {
  println!("file paths: {:?}", file_paths);
  println!("Adding specific folders...");
  let window = window.clone();
  // Spawn the new task
  tokio::spawn(async move {
    let mut conn = establish_direct_connection_to_db();
    if file_paths.len() > 0 {
      // This is when user manually adds folder/file(s)
      let file_paths_clone = file_paths.clone();
      for path in file_paths {
        let _ = add_path_to_allow_list(path, is_folder, &mut conn);
      }
      let _files_added = walk_directory(&mut conn, &window, file_paths_clone);
    } else {
      // This is the onboarding run
      let mut file_paths = file_paths;
      let home_directory = get_home_directory().unwrap();
      file_paths.push(home_directory);
      let _files_added = walk_directory(&mut conn, &window, file_paths);
    }
  });
}