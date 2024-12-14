use std::{fs, io, io::Write};
use std::path::Path;
use diesel::SqliteConnection;
use tauri_plugin_global_shortcut::Modifiers;
use crate::database::search::{get_parsed_text_for_file, get_file_id_from_path};
use crate::db_sync::sync_status;
use crate::housekeeping::get_app_directory;
use crate::indexing::extract_text_from_path;
use crate::user_prefs::set_scan_running_status;
use std::process::Command;
use crate::custom_types::Error;
use log::info;

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

pub fn _install_textra_from_github() -> Result<String, Error> {
  #[cfg(not(target_os = "macos"))]
  {
    Ok("Textra is only supported on MacOS".to_string())
  }
  #[cfg(target_os = "macos")]
  {
    let download_uri = "https://github.com/freedmand/textra/releases/download/0.2.1/textra-0.2.1.zip";
    let app_directory = get_app_directory();

    // check if textra is already installed
    let textra_check = Command::new("bash")
      .arg("-c")
      .arg(format!("cd {} && ./textra -v", app_directory))
      .output()
      .expect("Failed to execute command");
    // textra prints version number to stderr for some reason
    let textra_check_stderr = String::from_utf8(textra_check.stderr).unwrap();
    info!("Textra check stderr: {}", textra_check_stderr);

    if textra_check_stderr.contains("0.2.1") {
      return Ok("Textra is already installed".to_string());
    } else {
      // download textra
      let download_script = format!(
        r#"
        cd "{}"
        curl -L -o textra-0.2.1.zip {}
        "#,
        app_directory, download_uri
      );
      let download_output = Command::new("bash")
        .arg("-c")
        .arg(download_script)
        .output()
        .expect("Failed to execute command");
      let download_stdout = String::from_utf8(download_output.stdout).unwrap();
      let download_stderr = String::from_utf8(download_output.stderr).unwrap();
      info!("Textra download stdout: {}", download_stdout);
      info!("Textra download stderr: {}", download_stderr);

      // unzip textra
      let bash_script = format!(
        r#"
        cd "{}"
        unzip textra-0.2.1.zip
        chmod +x textra
        rm textra-0.2.1.zip
        ./textra -v
        "#,
        app_directory
      );
      let output = Command::new("bash")
        .arg("-c")
        .arg(bash_script)
        .output()
        .expect("Failed to execute command");
      let stdout = String::from_utf8(output.stdout).unwrap();
      let stderr = String::from_utf8(output.stderr).unwrap();
      info!("Textra installation stdout: {}", stdout);
      info!("Textra installation stderr: {}", stderr);
      Ok(stderr)
    }
  }
}

#[cfg(target_os = "windows")]
pub async fn install_poppler_from_github(handle: tauri::AppHandle) -> Result<String, Error> {
  #[cfg(not(target_os = "windows"))]
  {
    Ok("WinOCR is only supported on Windows".to_string())
  }
  #[cfg(target_os = "windows")]
  {
    let download_uri = "https://github.com/oschwartz10612/poppler-windows/releases/download/v24.02.0-0/Release-24.02.0-0.zip";
    let app_directory = get_app_directory();
    // check if poppler exe exists
    let poppler_path = format!("{}\\poppler-24.02.0\\Library\\bin\\pdftoppm.exe", app_directory);
    let poppler_exists = Path::new(&poppler_path).exists();
    println!("Poppler exists: {}", poppler_exists);
    if poppler_exists {
      println!("Poppler is already installed");
      return Ok("Poppler is already installed".to_string());
    } else {
      println!("Downloading and installing Poppler");
      // download poppler using reqwest
      let app_directory_clone = app_directory.clone();
      // let response = reqwest::get(download_uri).await.unwrap();
      // let mut file = std::fs::File::create(format!("{}\\poppler.zip", &app_directory_clone)).unwrap();
      // let mut content =  std::io::Cursor::new(response.bytes().await.unwrap());
      // std::io::copy(&mut content, &mut file).unwrap();

      let zip_file_path = handle.path().resolve("poppler.zip", BaseDirectory::Resource)?;
      // copy zip_file to app_directory
      fs::copy(zip_file_path, format!("{}/poppler.zip", &app_directory)).unwrap();

      // unzip poppler.zip using zip crate
      let mut archive = zip::ZipArchive::new(fs::File::open(format!("{}/poppler.zip", &app_directory)).unwrap()).unwrap();
      archive.extract(&app_directory).unwrap();
      // remove poppler.zip
      fs::remove_file(format!("{}\\poppler.zip", &app_directory)).unwrap();
      // check if poppler exe exists
      let poppler_path = format!("{}\\poppler-24.02.0\\Library\\bin\\pdftoppm.exe", &app_directory);
      if Path::new(&poppler_path).exists() {
        return Ok("Poppler is installed".to_string());
      } else {
        return Ok("Failed to install Poppler".to_string());
      }
    }
  }
}

pub async fn extract_text_from_pdf(file_path: String, conn: &mut SqliteConnection, app: &tauri::AppHandle) -> Result<Vec<String>, Error> {
  // check if file_path's text already exists in the tantivy index by calling get_parsed_text_for_file
  let mut text = vec![];
  // first, get the file's ID from the document table in the database
  let file_id = get_file_id_from_path(&file_path, conn).unwrap();
  if file_id > 0 {
    text = get_parsed_text_for_file(file_id, conn).unwrap();
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
  let mut file_path = file_path;
  if file_path == "scratchpad".to_string() {
    file_path = format!("{}/scratchpad.txt", get_app_directory());
  }
  let mut file = fs::File::create(file_path).unwrap();
  file.write_all(text.as_bytes()).unwrap();
}

pub async fn read_text_from_file(file_path: String) -> Result<String, Error> {
  // Reads text from a given .txt file path
  let mut file_path = file_path;
  if file_path == "scratchpad".to_string() {
    file_path = format!("{}/scratchpad.txt", get_app_directory());
  }
  let text = fs::read_to_string(file_path).unwrap();
  Ok(text)
}

pub async fn read_image_to_base64(file_path: String) -> Result<String, Error> {
  use base64::prelude::*;
  let image = fs::read(file_path).unwrap();
  let base64_image = BASE64_STANDARD.encode(&image);
  Ok(base64_image)
}