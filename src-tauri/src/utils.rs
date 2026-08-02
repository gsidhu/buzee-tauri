use std::{fs, io, io::Write};
use std::path::Path;
use diesel::SqliteConnection;
use tauri_plugin_global_shortcut::Modifiers;
use crate::database::search::{get_parsed_text_for_file, get_file_id_from_path};
use crate::db_sync::sync_status;
use crate::indexing::extract_text_from_path;
use crate::user_prefs::set_scan_running_status;
use crate::custom_types::Error;

pub fn get_metadata(path: &Path) -> io::Result<fs::Metadata> {
  // println!("Getting metadata for path: {:?}", path);
  let metadata = fs::metadata(path)?;
  Ok(metadata)
}

pub fn norm(path: &str) -> String {
  #[cfg(target_os = "windows")]
  {
    str::replace(path, "/", "\\")
  }
  
  #[cfg(target_os = "macos")]
  {
    str::replace(path, "\\", "/")
  }
}

pub fn string_to_modifiers(modifier: &str) -> Modifiers {
  match modifier {
    "ALT" => Modifiers::ALT,
    "ALT_GRAPH" => Modifiers::ALT_GRAPH,
    "CAPS_LOCK" => Modifiers::CAPS_LOCK,
    "CONTROL" => Modifiers::CONTROL,
    "FN" => Modifiers::FN,
    "FN_LOCK" => Modifiers::FN_LOCK,
    "META" => Modifiers::META,
    "NUM_LOCK" => Modifiers::NUM_LOCK,
    "SCROLL_LOCK" => Modifiers::SCROLL_LOCK,
    "SHIFT" => Modifiers::SHIFT,
    "SYMBOL" => Modifiers::SYMBOL,
    "SYMBOL_LOCK" => Modifiers::SYMBOL_LOCK,
    "HYPER" => Modifiers::HYPER,
    "SUPER" => Modifiers::SUPER,
    _ => Modifiers::empty()
  }
}

pub fn graceful_restart(app: tauri::AppHandle, conn: &mut SqliteConnection, wait_time: u64) {
  let sync_running = sync_status(&app);
  
  // if sync is running, wait for it to finish
  if sync_running.0 == "true" {
    set_scan_running_status(conn, false, true, &app);
    std::thread::sleep(std::time::Duration::from_secs(wait_time));
  }
  
  // restart the app
  app.restart();
}

pub async fn extract_text_from_pdf(file_path: String, conn: &mut SqliteConnection, app: &tauri::AppHandle) -> Result<Vec<String>, Error> {
  // check if file_path's text already exists in the tantivy index by calling get_parsed_text_for_file
  let mut text = vec![];
  // first, get the file's ID from the document table in the database
  let file_id = get_file_id_from_path(&file_path, conn).unwrap_or(0);
  if file_id > 0 {
    text = get_parsed_text_for_file(file_id, conn).unwrap_or_default();
  } 

  if text.is_empty() {
    // otherwise call extract_text_from_path
    let extracted_text = extract_text_from_path(file_path, "pdf".to_string(), app).await;
    // break extracted_text at line breaks and insert into text Vector
    text = extracted_text.split("\n").map(|s| s.to_string()).collect();
  }
  Ok(text)
}

pub async fn save_text_to_file(file_path: String, text: String) {
  let mut file = fs::File::create(file_path).unwrap();
  file.write_all(text.as_bytes()).unwrap();
}

pub async fn read_text_from_file(file_path: String) -> Result<String, Error> {
  // Reads text from a given .txt file path
  let text = fs::read_to_string(file_path).unwrap();
  Ok(text)
}

pub async fn read_image_to_base64(file_path: String) -> Result<String, Error> {
  use base64::prelude::*;
  let image = fs::read(file_path).unwrap();
  let base64_image = BASE64_STANDARD.encode(&image);
  Ok(base64_image)
}