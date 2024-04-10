use dotext::*;
use std::io::Read;
use std::error::Error;

pub fn extract(file: &String, _app: &tauri::AppHandle) -> Result<String, Box<dyn Error>> {
  /* Usage of ? at the end of the open() method call implies that if there is an error during method execution, then halt the execution and return the error. */ 
  let mut file_buffer = Docx::open(file)?;
  let mut text = String::new();
  file_buffer.read_to_string(&mut text)?;
  Ok(text)
}