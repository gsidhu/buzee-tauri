use std::error::Error;
use tauri_plugin_shell::ShellExt;
use crate::housekeeping::get_app_directory;
use crate::text_extraction::txt;

pub fn extract(file: &String, app: &tauri::AppHandle) -> Result<String, Box<dyn Error>> {
  // run textra on the file and save the output to a temporary file
  let app_directory = get_app_directory();

  #[cfg(target_os = "macos")]
  {
    let output_path = format!("{}/temp_output.txt", app_directory);
    // run textra on the file
    let sidecar_command = app.shell().sidecar("textra").unwrap().args([file, "-o", output_path.as_str()]);
    let (mut _rx, mut _child) = sidecar_command.spawn().unwrap();

    // read the temporary file
    let temp_file_path = format!("{}/temp_output.txt", app_directory);
    let text = txt::extract(&temp_file_path, app).unwrap();
    
    // return the extracted text
    return Ok(text)
  }
}
