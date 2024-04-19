use std::fs;
use std::io;
use std::path::Path;
use diesel::SqliteConnection;
use tauri_plugin_global_shortcut::Modifiers;
use crate::housekeeping::get_app_directory;
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

pub fn graceful_restart(app: tauri::AppHandle, conn: &mut SqliteConnection) {
  set_scan_running_status(conn, false, true, &app);
  // wait for 3 seconds
  std::thread::sleep(std::time::Duration::from_secs(3));
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

pub async fn install_poppler_from_github() -> Result<String, Error> {
  #[cfg(not(target_os = "windows"))]
  {
    Ok("Textra is only supported on Windows".to_string())
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
      let response = reqwest::get(download_uri).await.unwrap();
      let mut file = std::fs::File::create(format!("{}\\poppler.zip", &app_directory_clone)).unwrap();
      let mut content =  std::io::Cursor::new(response.bytes().await.unwrap());
      std::io::copy(&mut content, &mut file).unwrap();

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