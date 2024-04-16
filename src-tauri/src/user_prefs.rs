// Handles for User Preferences and App Data

use crate::custom_types::{GlobalShortcutState, SyncRunningState};
use crate::database::schema::user_preferences::global_shortcut_enabled;
use std::time::{SystemTime, UNIX_EPOCH};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use crate::database::models::{AppData, UserPrefs, FileTypes};
use crate::database::schema::{app_data, user_preferences, file_types};
use crate::database::establish_connection;
use crate::utils::string_to_modifiers;
use std::sync::Mutex;
use tauri::Manager;

pub fn set_default_user_prefs(conn: &mut SqliteConnection) {
  // get the first row from user_prefs table
  let existing_prefs = user_preferences::table
    .select(user_preferences::id)
    .load::<i32>(conn)
    .expect("Error loading user_prefs");

  if existing_prefs.len() == 0 {
    // insert default user_prefs
    let new_user_prefs = UserPrefs {
      first_launch_done: false,
      onboarding_done: false,
      launch_at_startup: true,
      show_in_dock: true,
      global_shortcut_enabled: true,
      global_shortcut: "Alt+Space".to_string(),
      automatic_background_sync: true,
      detailed_scan: true,
      disallowed_paths: "".to_string(),
    };

    // insert new_user_prefs into the user_prefs table
    diesel::insert_into(user_preferences::table)
      .values(&new_user_prefs)
      .execute(conn)
      .expect("Error inserting new user_prefs");
  }
}

pub fn set_default_app_data(conn: &mut SqliteConnection) {
  // get the first row from app_data table
  let existing_app_data = app_data::table
    .select(app_data::id)
    .load::<i32>(conn)
    .expect("Error loading app_data");

  if existing_app_data.len() == 0 {
    // insert default app_data
    let new_app_data = AppData {
      app_name: "Buzee".to_string(),
      app_version: "0.1.0".to_string(),
      app_mode: "window".to_string(),
      app_theme: "light".to_string(),
      app_language: "en".to_string(),
      last_scan_time: 0,
      scan_running: false
    };

    // insert new_app_data into the app_data table
    diesel::insert_into(app_data::table)
      .values(&new_app_data)
      .execute(conn)
      .expect("Error inserting new app_data");
  }
}

pub fn set_default_file_types(conn: &mut SqliteConnection) {
  const DOCUMENT_FILETYPES: [&str; 11] = ["csv", "docx", "key", "md", "numbers", "pages", "pdf", "pptx", "txt", "xlsx", "xls"];
  const IMAGE_FILETYPES: [&str; 4] = ["jpg", "jpeg", "png", "gif"];
  const BOOK_FILETYPES: [&str; 4] = ["epub", "mobi", "azw3", "pdf"];
  const AUDIO_FILETYPES: [&str; 5] = ["mp3", "wav", "aac", "flac", "ogg"];
  const VIDEO_FILETYPES: [&str; 5] = ["mp4", "mkv", "avi", "mov", "wmv"];

  // get the first row from file_types table
  let existing_file_types = file_types::table
    .select(file_types::id)
    .load::<i32>(conn)
    .expect("Error loading file_types");

  if existing_file_types.len() == 0 {
    let mut new_file_types: Vec<FileTypes> = Vec::new();
    // create FileTypes structs and add them to the new_file_types vector
    for file_type in DOCUMENT_FILETYPES.iter() {
      let new_file_type = FileTypes {
        file_type: file_type.to_string(),
        file_type_category: "document".to_string(),
        file_type_allowed: true,
        added_by_user: false,
      };
      new_file_types.push(new_file_type);
    }
    for file_type in IMAGE_FILETYPES.iter() {
      let new_file_type = FileTypes {
        file_type: file_type.to_string(),
        file_type_category: "image".to_string(),
        file_type_allowed: true,
        added_by_user: false,
      };
      new_file_types.push(new_file_type);
    }
    for file_type in BOOK_FILETYPES.iter() {
      let new_file_type = FileTypes {
        file_type: file_type.to_string(),
        file_type_category: "book".to_string(),
        file_type_allowed: true,
        added_by_user: false,
      };
      new_file_types.push(new_file_type);
    }
    for file_type in AUDIO_FILETYPES.iter() {
      let new_file_type = FileTypes {
        file_type: file_type.to_string(),
        file_type_category: "audio".to_string(),
        file_type_allowed: true,
        added_by_user: false,
      };
      new_file_types.push(new_file_type);
    }
    for file_type in VIDEO_FILETYPES.iter() {
      let new_file_type = FileTypes {
        file_type: file_type.to_string(),
        file_type_category: "video".to_string(),
        file_type_allowed: true,
        added_by_user: false,
      };
      new_file_types.push(new_file_type);
    }

    // Add folder filetype
    let new_file_type = FileTypes {
      file_type: "folder".to_string(),
      file_type_category: "folder".to_string(),
      file_type_allowed: true,
      added_by_user: false,
    };
    new_file_types.push(new_file_type);

    // insert all new file_types into the file_types table using a transaction
    conn
      .transaction::<_, diesel::result::Error, _>(|conn| {
          diesel::insert_into(file_types::table)
              .values(new_file_types)
              .execute(conn)
      })
      .unwrap();
  }
}

pub fn set_scan_running_status(conn: &mut SqliteConnection, status: bool, set_time: bool, app: &tauri::AppHandle) {
  println!("Setting scan_running status to: {}", status);
  let state_mutex = app.state::<Mutex<SyncRunningState>>();
  let mut state = state_mutex.lock().unwrap();
  state.sync_running = status;

  if set_time {
    state.last_sync_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    
    let _ = diesel::update(app_data::table)
      .set((
          app_data::scan_running.eq(&status),
          app_data::last_scan_time.eq(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64)
      ))
      .execute(conn)
      .unwrap();
  } else {
    let _ = diesel::update(app_data::table)
      .set(app_data::scan_running.eq(&status))
      .execute(conn)
      .unwrap();
  }
}

// Global Shortcut Functions
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};
use std::str::FromStr;

pub fn set_new_global_shortcut_in_db(new_shortcut_string: String, app: &tauri::AppHandle) {
  let mut conn = establish_connection(&app);
  let _ = diesel::update(user_preferences::table)
    .set(user_preferences::global_shortcut.eq(new_shortcut_string))
    .execute(&mut conn)
    .unwrap();
}

pub fn set_global_shortcut_flag_in_db(flag: bool, app: &tauri::AppHandle) {
  let mut conn = establish_connection(&app);
  let _ = diesel::update(user_preferences::table)
    .set(user_preferences::global_shortcut_enabled.eq(flag))
    .execute(&mut conn)
    .unwrap();
}

pub fn set_global_shortcut_state_from_db_value(app: &tauri::AppHandle) {
  println!("Setting global shortcut to mutex state");
  let state_mutex = app.state::<Mutex<GlobalShortcutState>>();
  let mut state = state_mutex.lock().unwrap();

  let mut conn = establish_connection(&app);
  // get global_shortcut_enabled and global_shortcut values from user_preferences table
  let global_shortcut_vals = user_preferences::table
    .select((global_shortcut_enabled, user_preferences::global_shortcut))
    .first::<(bool, String)>(&mut conn)
    .expect("Error loading user_prefs");

  state.shortcut_enabled = global_shortcut_vals.0;
  state.shortcut_string = global_shortcut_vals.1.to_string();
  println!("Set global shortcut mutex state to: {}", state.shortcut_string);
}

pub fn is_global_shortcut_enabled(app: &tauri::AppHandle) -> bool {
  let state_mutex = app.state::<Mutex<GlobalShortcutState>>();
  let state = state_mutex.lock().unwrap();
  state.shortcut_enabled
}

pub fn get_global_shortcut(app: &tauri::AppHandle) -> Shortcut {
  let (combined_modifiers, key) = set_modifiers_and_code_from_state(app);
  let shortcut_modifiers = combined_modifiers.iter().fold(Some(combined_modifiers[0]), |acc, &modifier| {
    Some(match acc {
        Some(acc_value) => acc_value | modifier,
        None => modifier,
    })
  });
  Shortcut::new(shortcut_modifiers, key)
}

pub fn get_modifiers_and_code_from_global_shortcut(app: &tauri::AppHandle) -> (Modifiers, Code) {
  let (combined_modifiers, key) = set_modifiers_and_code_from_state(app);
  let shortcut_modifiers = combined_modifiers.iter().fold(Some(combined_modifiers[0]), |acc, &modifier| {
    Some(match acc {
        Some(acc_value) => acc_value | modifier,
        None => modifier,
    })
  }).unwrap();

  (shortcut_modifiers, key)
}

fn set_modifiers_and_code_from_state(app: &tauri::AppHandle) -> (Vec<Modifiers>, Code) {
  let state_mutex = app.state::<Mutex<GlobalShortcutState>>();
  let state = state_mutex.lock().unwrap();
  let global_shortcut_string = &state.shortcut_string;
  println!("global_shortcut_string2: {:?}", global_shortcut_string);
  let mut splits: Vec<&str> = global_shortcut_string.split("+").collect();
  let key = Code::from_str(splits.last().unwrap()).unwrap();
  let _ = splits.pop();
  let mut combined_modifiers: Vec<Modifiers> = Vec::new();
  for modifier in &splits {
    let uppercase = modifier.to_uppercase();
    combined_modifiers.push(string_to_modifiers(&uppercase));
  }
  (combined_modifiers, key)
}