use std::error::Error;
use tauri_plugin_shell::{ShellExt, process::CommandEvent};
use crate::housekeeping::get_app_directory;
use crate::text_extraction::txt;

pub async fn extract(file: &String, app: &tauri::AppHandle) -> Result<String, Box<dyn Error>> {
  // run textra on the file and save the output to a temporary file
  let app_directory = get_app_directory();

  #[cfg(target_os = "macos")]
  {
    let output_path = format!("{}/temp_output.txt", app_directory);

    // run textra on the file
    let sidecar_command = app.shell().sidecar("textra").unwrap().args([file, "-o", output_path.as_str()]);
    let (mut rx, mut _child) = sidecar_command.spawn().unwrap();

    // let mut text = String::new();
    while let Some(event) = rx.recv().await {
      // LOGIC:
      // textra prints only the text to stdout, everything else goes to stderr
      // but since we define an output file, there is no stdout
      // so we just poll the stderr to keep the loop running till the extraction completes
      if let CommandEvent::Stderr(line) = event {
        let _output_line = String::from_utf8(line).unwrap();
        // println!("text: {}", output_line);
      }
    }

    // read the temporary file
    let temp_file_path = format!("{}/temp_output.txt", app_directory);
    let text = txt::extract(&temp_file_path, app)?;

    // return the extracted text
    return Ok(text)
  }

  #[cfg(target_os = "windows")]
  {
    let output_path = format!("{}/temp_output.txt", app_directory);

    // run textra on the file
    let sidecar_command = app.shell().sidecar("winocr").unwrap().args(["-i", file, "-o", output_path.as_str()]);
    let (mut rx, mut _child) = sidecar_command.spawn().unwrap();

    // let mut text = String::new();
    while let Some(event) = rx.recv().await {
      // LOGIC:
      // textra prints only the text to stdout, everything else goes to stderr
      // but since we define an output file, there is no stdout
      // so we just poll the stderr to keep the loop running till the extraction completes
      if let CommandEvent::Stderr(line) = event {
        let _output_line = String::from_utf8(line).unwrap();
        // println!("text: {}", output_line);
      }
    }

    // read the temporary file
    let temp_file_path = format!("{}/temp_output.txt", app_directory);
    let text = txt::extract(&temp_file_path, app)?;

    // return the extracted text
    return Ok(text)
  }
  
}
