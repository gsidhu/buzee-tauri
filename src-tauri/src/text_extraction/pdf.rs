use std::collections::BTreeMap;
use std::error::Error;

use lopdf::Document;

pub fn extract(file: &String) -> Result<String, Box<dyn Error>> {
  let document: Document = Document::load(file)?;
  let pages: BTreeMap<u32, (u32, u16)> = document.get_pages();
  let mut texts: Vec<String> = Vec::new();

  for (i, _) in pages.iter().enumerate() {
    let page_number: u32 = (i + 1) as u32;
    let text: String = document.extract_text(&[page_number])?;
    texts.push(text);
  }

  let text: String = texts.join("\n");
  Ok(text)
}