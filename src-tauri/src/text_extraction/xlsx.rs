use dotext::*;
use std::io::Read;
use std::error::Error;

pub fn extract(file: &String) -> Result<String, Box<dyn Error>> {
  let mut file_buffer = Xlsx::open(file)?;
  let mut text = String::new();
  file_buffer.read_to_string(&mut text)?;
  Ok(text)
}