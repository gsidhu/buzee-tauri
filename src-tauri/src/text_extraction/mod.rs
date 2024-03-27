use std::collections::HashMap;
use std::error::Error;

pub mod csv;
// pub mod docx;
pub mod epub;
pub mod mobi;
pub mod pdf;
pub mod pptx;
pub mod txt;
pub mod xlsx;

pub struct Extractor {
  map: HashMap<String, fn(&String) -> Result<String, Box<dyn Error>>>,
}

impl Extractor {
  pub fn new() -> Self {
    let mut map: HashMap<String, fn(&String) -> Result<String, Box<dyn Error>>> = HashMap::new();
    map.insert("csv".to_string(), csv::extract);
    // map.insert("docx".to_string(), docx::extract);
    map.insert("epub".to_string(), epub::extract);
    map.insert("mobi".to_string(), mobi::extract);
    map.insert("md".to_string(), txt::extract);
    map.insert("pdf".to_string(), pdf::extract);
    map.insert("pptx".to_string(), pptx::extract);
    map.insert("txt".to_string(), txt::extract);
    map.insert("xlsx".to_string(), xlsx::extract);
    Extractor { map }
  }

  pub fn extract_text_from_file(
    &self,
    file_path: String,
    file_type: String,
  ) -> Result<String, Box<dyn Error>> {
    println!("Extracting text from file: {}", file_path);
    if let Some(extract) = self.map.get(&file_type) {
      extract(&file_path)
    } else {
      Err("File type not supported".into())
    }
  }
}
