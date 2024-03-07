use lopdf::Document;

pub fn extract(file: &String) -> String {
  match Document::load(file) {
    Ok(document) => {
      let pages = document.get_pages();
      let mut texts = Vec::new();

      for (i, _) in pages.iter().enumerate() {
        let page_number = (i + 1) as u32;
        let text = document.extract_text(&[page_number]);
        texts.push(text.unwrap_or_default());
      }

      let text = texts.join("\n");
      // println!("Text on page {}: {}", 3, texts[2]);
      text
    }
    Err(err) => {
      eprintln!("Error: {:?}", err);
      String::new()
    }
  }
}