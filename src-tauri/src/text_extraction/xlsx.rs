use dotext::*;
use std::io::Read;

pub fn extract(file: &String) -> String {
  let mut file_buffer = Xlsx::open(file).expect("Cannot open file");
  let mut text = String::new();
  let _ = file_buffer.read_to_string(&mut text);
  // println!("Text: {}", text);
  text
}