// Inter-Process Communication between Rust and SvelteKit
// The idea is to eventually keep only callers here and move the actual logic to other files. This way, for creating a web app, we just have to convert this file into an API and call the same functions from the frontend.

use crate::custom_types::{ContextMenuState, DBConnPoolState, DBStat, DateLimit, Error, Payload, SyncRunningState, TantivyBookmarkSearchResult, TantivyDocumentSearchResult, TantivyReaderState, UserPreferencesState};
use crate::database::{establish_connection, get_connection_pool};
use crate::database::models::{DocumentSearchResult, IgnoreList};
use crate::database::search::{
    get_counts_for_all_filetypes, get_metadata_title_matches, get_parsed_text_for_file, get_recently_opened_docs, get_file_parsed_count, search_fts_index
};
use crate::db_sync::{run_sync_operation, sync_status, add_specific_folders};
use crate::indexing::{add_path_to_ignore_list, all_allowed_filetypes, get_all_ignored_paths, remove_nonexistent_and_ignored_files, remove_paths_from_ignore_list};
use crate::user_prefs::{fix_global_shortcut_string, get_global_shortcut, get_modifiers_and_code_from_global_shortcut, is_global_shortcut_enabled, return_user_prefs_state, set_automatic_background_sync_flag_in_db, set_default_user_prefs, set_detailed_scan_flag_in_db, set_roadmap_survey_answered_flag_in_db, set_global_shortcut_flag_in_db, set_launch_at_startup_flag_in_db, set_new_global_shortcut_in_db, set_onboarding_done_flag_in_db, set_show_search_suggestions_flag_in_db, set_user_preferences_state_from_db_value};
use crate::utils::{extract_text_from_pdf, graceful_restart, read_image_to_base64, read_text_from_file, save_text_to_file};
use crate::window::hide_or_show_window;
use serde_json;
use tauri::Manager;
use tauri_plugin_shell;
use tokio::time::{interval, Duration};
use tauri::menu::Menu;
use crate::context_menu::{contextmenu_receiver, searchresult_context_menu_folder, searchresult_context_menu_docs, searchresult_context_menu_other, statusbar_context_menu, tableheader_context_menu};
// use log::info;
use std::sync::Mutex;
use std::process::Command;
use crate::tantivy_index::{delete_all_docs_from_index, acquire_searcher_from_reader, create_tantivy_schema, get_reader_for_index, get_tantivy_index, parse_query_and_get_top_docs, return_bookmark_search_results, return_document_search_results};
use crate::tantivy_index::internal_test_create_csv_dump_from_index;
use tauri::Emitter;

#[cfg(target_os = "windows")]
use dirs::home_dir;

pub fn send_message_to_frontend(
    window: &tauri::WebviewWindow,
    event: String,
    message: String,
    data: String,
) {
  window.emit(&event, Payload { message, data }).unwrap();
}

// Setup cron job for background sync
#[tauri::command]
async fn setup_cron_job(window: tauri::WebviewWindow, app: tauri::AppHandle) {
  let app_clone = app.clone();
  let state_mutex = app_clone.state::<Mutex<UserPreferencesState>>();
  let state = state_mutex.lock().unwrap();
  if state.automatic_background_sync {
    tokio::spawn(async move {
      let mut interval = interval(Duration::from_secs(1800));
      // first tick happens immediately, so get it out of the way
      interval.tick().await;
      loop {
        interval.tick().await;
        let sync_running = sync_status(&app);
        println!("??? Sync running: {}", sync_running.0);
        let current_timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
        if sync_running.0 == "false" && current_timestamp - sync_running.1 > 1800 {
          let window_clone = window.clone();
          let app_clone = app.clone();
          run_sync_operation(window_clone, app_clone, false, Vec::new()).await;
        }
      }
    });
  }
}

#[tauri::command]
async fn polite_restart(app: tauri::AppHandle) {
  let mut conn = establish_connection(&app);
  graceful_restart(app, &mut conn, 30);
}

// Get OS boolean
#[tauri::command]
fn get_os() -> Result<String, Error> {
    let os = std::env::consts::OS;
    Ok(os.to_string())
}

// Get allowed filetypes
#[tauri::command]
fn get_allowed_filetypes(app: tauri::AppHandle) -> Result<String, Error> {
  let mut connection = establish_connection(&app);
  let allowed_filetypes = all_allowed_filetypes(&mut connection, true);
  // Convert allowed_filetypes to JSON using serde_json
  let json_response = serde_json::to_string(&allowed_filetypes).unwrap();
  Ok(json_response)
}

// Open a file (in default app) or a folder from the path
#[tauri::command]
fn open_file_or_folder(file_path: String, window: tauri::Window) -> Result<String, Error> {
    println!(
        "Window {} invoked this command to open {}",
        window.label(),
        file_path
    );
    let _ = open::that(file_path);
    // If it worked
    Ok("Opened the file or folder!".into())
}

// Open the folder containing the file from the filepath
#[tauri::command]
fn open_folder_containing_file(file_path: String) -> Result<String, Error> {
    println!("Opening folder for {}", file_path);
    let do_steps = || -> Result<(), Error> {
        #[cfg(target_os = "windows")]
        {
          // This opens the folder in the background, which is not desirable
          // Maybe remove it in future
          Command::new("explorer")
          .args(["/select,", &file_path]) // The comma after select is not a typo
          .spawn()
          .map_err(|e| e)?;
        }

        #[cfg(target_os = "macos")]
        {
          let path_buf = std::path::PathBuf::from(&file_path);
          if path_buf.is_dir() {
          Command::new("open")
            .args([&file_path])
            .spawn()
            .map_err(|e| e)?;
          } else {
          Command::new("open")
            .args(["-R", &file_path])
            .spawn()
            .map_err(|e| e)?;
          }
        }
        Ok(())
    };

    if let Err(_err) = do_steps() {
        let path = std::path::PathBuf::from(file_path);
        let dir = path.parent().unwrap();
        let _ = open::that_detached(dir);
    }
    Ok("Opened the folder!".into())
}

// Run file indexing ONLY
#[tauri::command]
async fn run_file_indexing(window: tauri::WebviewWindow, file_paths: Vec<String>, is_folder: bool) -> Result<String, Error> {
  println!("File watcher started");
  add_specific_folders(&window, file_paths, is_folder).await;
  Ok("File indexing complete".to_string())
}

// Run file sync
#[tauri::command]
async fn run_file_sync(switch_off: bool, app: tauri::AppHandle, window: tauri::WebviewWindow, file_paths: Vec<String>) {
  run_sync_operation(window, app, switch_off, file_paths).await;
}

// Ignore file or folder path
#[tauri::command]
async fn ignore_file_or_folder(app: tauri::AppHandle, path: String, is_directory: bool, should_ignore_indexing: bool, should_ignore_content: bool) {
  let mut conn = establish_connection(&app);
  add_path_to_ignore_list(path, is_directory, should_ignore_indexing, should_ignore_content, &mut conn).unwrap();
  remove_nonexistent_and_ignored_files(&mut conn);
}

// Remove list of paths from Ignore List
#[tauri::command]
async fn remove_from_ignore_list(app: tauri::AppHandle, paths: Vec<String>) {
  let mut conn = establish_connection(&app);
  let _ = remove_paths_from_ignore_list(paths, &mut conn);
}

// Ignore file or folder path
#[tauri::command]
async fn show_ignored_paths(app: tauri::AppHandle) -> Result<Vec<IgnoreList>, Error> {
  let mut conn = establish_connection(&app);
  Ok(get_all_ignored_paths(&mut conn))
}

// Get sync status
#[tauri::command]
fn get_sync_status(app: tauri::AppHandle) -> Result<String, Error> {
  let sync_running = sync_status(&app).0;
  Ok(sync_running)
}

// Get search suggestions
#[tauri::command]
fn get_search_suggestions(query: String, app: tauri::AppHandle) -> Result<Vec<String>, Error> {
  let mut conn = establish_connection(&app);
  let suggestions = get_metadata_title_matches(query, &mut conn).unwrap_or(vec![]);
  Ok(suggestions)
}

// Run search
#[tauri::command]
fn run_search(query: String, page: i32, limit: i32, file_type: Option<String>, date_limit: Option<DateLimit>, app: tauri::AppHandle) -> Result<Vec<DocumentSearchResult>, Error> {
    println!(
        "run_search: query: {}, page: {}, limit: {}, file_type: {:?}, date_limit: {:?}",
        query, page, limit, file_type, date_limit
    );
    let conn = establish_connection(&app);
    let search_results = search_fts_index(query, page, limit, file_type, date_limit, conn, &app).unwrap_or(vec![]);
    Ok(search_results)
}

// Get recently opened documents
#[tauri::command]
fn get_recent_docs(
    page: i32,
    limit: i32,
    file_type: Option<String>,
    app: tauri::AppHandle
) -> Result<Vec<DocumentSearchResult>, Error> {
    let conn = establish_connection(&app);
    let search_results = get_recently_opened_docs(page, limit, file_type, conn).unwrap();
    Ok(search_results)
}

// Get DB Stats
#[tauri::command]
fn get_db_stats(app: tauri::AppHandle) -> Result<Vec<DBStat>, Error> {
  let conn = establish_connection(&app);
  let db_stats = get_counts_for_all_filetypes(conn).unwrap();
  Ok(db_stats)
}

// Get Total Files and Files Parsed
#[tauri::command]
fn get_count_of_files_parsed(app: tauri::AppHandle) -> Result<i64, Error> {
  let conn = establish_connection(&app);
  let result = get_file_parsed_count(conn).unwrap();
  Ok(result)
}

// Get parsed text for file
#[tauri::command]
async fn get_text_for_file(document_id: i32, app: tauri::AppHandle) -> Result<Vec<String>, Error> {
    println!("Getting text for file ID: {}", document_id);
    let mut conn = establish_connection(&app);
    let text = get_parsed_text_for_file(document_id, &mut conn).unwrap();
    Ok(text)
}

// Extract text for PDF
#[tauri::command]
async fn extract_text_from_pdf_file(file_path: String, app: tauri::AppHandle) -> Result<Vec<String>, Error> {
  let mut conn = establish_connection(&app);
  let text = extract_text_from_pdf(file_path, &mut conn, &app).await.unwrap();
  Ok(text)
}

// Write text to file
#[tauri::command]
async fn write_text_to_file(file_path: String, text: String) {
  save_text_to_file(file_path, text).await;
}

// Read text from .txt file
#[tauri::command]
async fn read_text_from_txt_file(file_path: String) -> Result<String, Error>  {
  read_text_from_file(file_path).await
}

// Open QuickLook (MacOS) or Peek (Windows)
#[tauri::command]
fn open_quicklook(file_path: String) -> Result<String, Error> {
    println!("Opening QuickLook for {}", file_path);

    #[cfg(target_os = "macos")]
    std::thread::spawn(move || {
        let _ = std::process::Command::new("qlmanage")
            .arg("-p")
            .arg(file_path)
            .output();
    });

    #[cfg(target_os = "windows")]
    std::thread::spawn(move || {
      let home_directory = home_dir().unwrap().to_string_lossy().to_string();
      let quicklook_path = format!("{}\\AppData\\Local\\Programs\\QuickLook\\QuickLook.exe", &home_directory);
      let _ = std::process::Command::new(quicklook_path)
        .arg(file_path)
        .output();
    });
    Ok("Opened QuickLook!".into())
}

// Context Menu
#[tauri::command]
fn open_context_menu(app_handle: tauri::AppHandle, window: tauri::Window, option: String, filetype: String) {
  let state_mutex = app_handle.state::<Mutex<ContextMenuState>>();
  let state = state_mutex.lock().unwrap();
  match option.as_str() {
      "searchresult" => {
        let context_menu = match filetype.as_str() {
          "folder" => &state.folder,
          "docx" => &state.docs,
          "md" => &state.docs,
          "pptx" => &state.docs,
          "txt" => &state.docs,
          "epub" => &state.docs,
          "pdf" => &state.docs,
          _ => &state.other,
        };
        window.popup_menu(context_menu).unwrap();
      },
      "statusbar" => {
        let context_menu = &state.status_bar;
        window.popup_menu(context_menu).unwrap();
      }
      "tableheader" => {
        let context_menu = &state.table_header;
        window.popup_menu(context_menu).unwrap();
      },
      _ => println!("Invalid context menu option"),
  }
}

#[tauri::command]
async fn get_image_base64(file_path: String) -> Result<String, Error> {
  read_image_to_base64(file_path).await
}

// Get UserPreferencesState
#[tauri::command]
async fn get_user_preferences_state(app_handle: tauri::AppHandle) -> UserPreferencesState {
  return_user_prefs_state(&app_handle)
}

// Set user preference in DB and State
#[tauri::command]
async fn reset_user_preferences(app_handle: tauri::AppHandle) {
  let mut conn = establish_connection(&app_handle);
  set_default_user_prefs(&mut conn, true);
  set_user_preferences_state_from_db_value(&app_handle);
  graceful_restart(app_handle, &mut conn, 30);
}

#[tauri::command]
async fn set_user_preference(window: tauri::WebviewWindow, app_handle: tauri::AppHandle, key: String, value: bool) {
  println!("Setting {} to {}", key, value);
  let mut conn = establish_connection(&app_handle);
  match key.as_str() {
    "launch_at_startup" => {
      set_launch_at_startup_flag_in_db(value, &app_handle);
      set_user_preferences_state_from_db_value(&app_handle);
    }
    "show_search_suggestions" => {
      set_show_search_suggestions_flag_in_db(value, &app_handle);
      set_user_preferences_state_from_db_value(&app_handle);
    }
    "onboarding_done" => {
      set_onboarding_done_flag_in_db(value, &app_handle);
      set_user_preferences_state_from_db_value(&app_handle);
    }
    "automatic_background_sync" => {
      set_automatic_background_sync_flag_in_db(value, &app_handle);
      set_user_preferences_state_from_db_value(&app_handle);
      if value == true {
        setup_cron_job(window, app_handle).await;
      }
    }
    "detailed_scan" => {
      set_detailed_scan_flag_in_db(value, &app_handle);
      set_user_preferences_state_from_db_value(&app_handle);
    }
    "roadmap_survey_answered" => {
      set_roadmap_survey_answered_flag_in_db(value, &app_handle);
      set_user_preferences_state_from_db_value(&app_handle);
    }
    "global_shortcut_enabled" => {
      set_global_shortcut_flag_in_db(value, &app_handle);
      set_user_preferences_state_from_db_value(&app_handle);
      graceful_restart(app_handle, &mut conn, 30);
    }
    _ => {
      println!("Invalid key");
    } 
  }
}

// Set new global shortcut in DB and update the global shortcut
#[tauri::command]
async fn set_new_global_shortcut(app_handle: tauri::AppHandle, new_shortcut_string: String) {
  println!("Setting new global shortcut: {}", new_shortcut_string);
  let new_shortcut_string = fix_global_shortcut_string(new_shortcut_string);
  let mut conn = establish_connection(&app_handle);
  set_new_global_shortcut_in_db(new_shortcut_string, &app_handle);
  set_user_preferences_state_from_db_value(&app_handle);
  graceful_restart(app_handle, &mut conn, 30);
  // Tell user to restart the app for changes to take effect
  // restart the app to set the new shortcut
  // app_handle.restart();
}

// #[tauri::command]
// async fn run_sidecar(app: tauri::AppHandle) {
//   use tauri_plugin_shell::{ShellExt, process::CommandEvent};
//   let sidecar_command = app.shell().sidecar("test").unwrap().args(["buzeeeeee"]);
//   let (mut rx, mut _child) = sidecar_command.spawn().unwrap();
//   let mut text = String::new();
//   while let Some(event) = rx.recv().await {
//     if let CommandEvent::Stdout(line) = event {
//       let output_line = String::from_utf8(line).unwrap();
//       text += &output_line;
//       println!("sidecaaar text: {}", text);
//     }
//   }
// }

#[tauri::command]
fn search_tantivy_files_index(app_handle: tauri::AppHandle, user_query: String, limit: i32, page: i32) -> Result<Vec<TantivyDocumentSearchResult>, Error> {
  println!("Searching Tantivy index...");
  let tantivy_index = get_tantivy_index(create_tantivy_schema()).unwrap();
  let searcher = acquire_searcher_from_reader(&app_handle).unwrap();

  let top_docs = parse_query_and_get_top_docs(&tantivy_index, &searcher, user_query, limit, page*limit).unwrap();
  let search_results = return_document_search_results(&tantivy_index, &searcher, top_docs).unwrap_or(vec![]);
  Ok(search_results)
}

#[tauri::command]
fn search_tantivy_bookmarks_index(app_handle: tauri::AppHandle, user_query: String, limit: i32, page: i32) -> Result<Vec<TantivyBookmarkSearchResult>, Error> {
  println!("Searching Tantivy index...");
  let tantivy_index = get_tantivy_index(create_tantivy_schema()).unwrap();
  let searcher = acquire_searcher_from_reader(&app_handle).unwrap();

  let top_docs = parse_query_and_get_top_docs(&tantivy_index, &searcher, user_query, limit, page*limit).unwrap();
  let search_results = return_bookmark_search_results(&tantivy_index, &searcher, top_docs).unwrap_or(vec![]);
  Ok(search_results)
}

#[tauri::command]
fn create_csv_dump(app_handle: tauri::AppHandle) {
  let searcher = acquire_searcher_from_reader(&app_handle).unwrap();
  internal_test_create_csv_dump_from_index(&searcher);
}

#[tauri::command]
fn clear_index() {
  let _ = delete_all_docs_from_index();
}

pub fn initialize() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      setup_cron_job,
      polite_restart,
      get_allowed_filetypes,
      get_os,
      open_file_or_folder,
      open_folder_containing_file,
      run_file_indexing,
      run_file_sync,
      get_sync_status,
      get_search_suggestions,
      run_search,
      get_recent_docs,
      get_db_stats,
      get_count_of_files_parsed,
      get_text_for_file,
      extract_text_from_pdf_file,
      write_text_to_file,
      read_text_from_txt_file,
      open_quicklook,
      open_context_menu,
      set_user_preference,
      set_new_global_shortcut,
      crate::drag::start_drag,
      get_user_preferences_state,
      reset_user_preferences,
      ignore_file_or_folder,
      show_ignored_paths,
      remove_from_ignore_list,
      get_image_base64,
      search_tantivy_files_index,
      search_tantivy_bookmarks_index,
      create_csv_dump,
      clear_index
    ])
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    .plugin(tauri_plugin_dialog::init())
    .setup(|app| {
        {
          // manage state(s)
          let handle = app.handle();
          // db connection pool
          let pool = get_connection_pool();
          handle.manage(Mutex::new(DBConnPoolState::new(pool)));
          // tantivy reader state
          let tantivy_index = get_tantivy_index(create_tantivy_schema()).unwrap();
          let given_reader = get_reader_for_index(&tantivy_index).unwrap();
          handle.manage(Mutex::new(TantivyReaderState::new(given_reader)));
          // user preferences state
          handle.manage(Mutex::new(UserPreferencesState::default()));
          set_user_preferences_state_from_db_value(app.handle());
          // sync running state
          handle.manage(Mutex::new(SyncRunningState::default()));
          // context menu
          let main_window = handle.get_webview_window("main").unwrap();
          let folder_context_menu = searchresult_context_menu_folder(&main_window);
          let docs_context_menu = searchresult_context_menu_docs(&main_window);
          let other_context_menu = searchresult_context_menu_other(&main_window);
          let table_header_context_menu = tableheader_context_menu(&main_window);
          let status_bar_context_menu = statusbar_context_menu(&main_window);
          handle.manage(Mutex::new(ContextMenuState::new(folder_context_menu, docs_context_menu, other_context_menu, table_header_context_menu, status_bar_context_menu)));
        }
        {
          if is_global_shortcut_enabled(app.handle()) {
            println!("Global Shortcut is enabled");
            use tauri_plugin_global_shortcut::ShortcutState;
            app.handle().plugin(
              tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut(get_global_shortcut(app.handle()))?
                .with_handler(|app_handle, shortcut, event| {
                  if event.state == ShortcutState::Pressed {
                    let (global_shortcut_modifiers, global_shortcut_code) = get_modifiers_and_code_from_global_shortcut(app_handle);
                    println!("shortcut: {:?}", shortcut);
                    if shortcut.matches(global_shortcut_modifiers, global_shortcut_code) {
                      println!("Global Shortcut Detected!");
                      let main_window = app_handle.get_webview_window("main").unwrap();
                      hide_or_show_window(main_window);
                    }
                  }
                })
                .build(),
            )?;
          } else {
            println!("Global Shortcut is disabled");
          }
        }
        {
          app.on_menu_event(|app_handle: &tauri::AppHandle, event| {
            // println!("menu event: {:?}", event);
            contextmenu_receiver(app_handle, event);
          });
        }
        {
          // Set activation poicy to Accessory to prevent the app icon from showing on the dock
          // app.set_activation_policy(tauri::ActivationPolicy::Accessory);
        }
        Ok(())
    })
    .menu(|app_handle| Menu::default(app_handle))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}