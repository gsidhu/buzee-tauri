use dotext::*;
use std::io::Read;
use std::error::Error;

pub fn extract(file: &String, _app: &tauri::AppHandle) -> Result<String, Box<dyn Error>> {
  let mut file_buffer = Xlsx::open(file)?;
  let mut text = String::new();
  file_buffer.read_to_string(&mut text)?;
  // remove all numbers from the text
  text = text.chars().filter(|c| !c.is_numeric()).collect();
  // keep only unique words
  let words: Vec<&str> = text.split_whitespace().collect();
  let mut unique_words = Vec::new();
  for word in words {
    if !unique_words.contains(&word) {
      unique_words.push(word);
    }
  }
  Ok(text)
}