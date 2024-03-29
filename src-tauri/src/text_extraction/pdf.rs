use std::error::Error;
use std::process::Command;
use crate::housekeeping::get_app_directory;
use crate::text_extraction::txt;

pub fn extract(file: &String) -> Result<String, Box<dyn Error>> {
  // run textra on the file and save the output to a temporary file
  let app_directory = get_app_directory();

  #[cfg(target_os = "macos")]
  {
    let bash_script = format!(
      r#"
      cd {}
      ./textra -s "{}" -o temp_output.txt
      "#,
      app_directory, file
    );
    let textra_command = Command::new("bash")
      .arg("-c")
      .arg(bash_script)
      .output()
      .expect("Failed to execute command");
    let _textra_stdout = String::from_utf8(textra_command.stdout).unwrap();
    let _textra_stderr = String::from_utf8(textra_command.stderr).unwrap();
    // read the temporary file
    let temp_file_path = format!("{}/temp_output.txt", app_directory);
    let text = txt::extract(&temp_file_path).unwrap();
    // return the extracted text
    return Ok(text)
  }
}
