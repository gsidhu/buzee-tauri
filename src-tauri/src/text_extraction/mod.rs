use std::collections::HashMap;
use std::error::Error;

pub mod pdf;
pub mod docx;
pub mod pptx;
pub mod xlsx;
pub mod txt;

pub struct Extractor {
    map: HashMap<String, fn(&String) -> Result<String, Box<dyn Error>>>,
}

impl Extractor {
    pub fn new() -> Self {
        let mut map: HashMap<String, fn(&String) -> Result<String, Box<dyn Error>>> = HashMap::new();
        map.insert(".docx".to_string(), docx::extract);
        map.insert(".md".to_string(), txt::extract);
        map.insert(".pdf".to_string(), pdf::extract);
        map.insert(".pptx".to_string(), pptx::extract);
        map.insert(".txt".to_string(), txt::extract);
        map.insert(".xlsx".to_string(), xlsx::extract);
        Extractor { map }
    }

    pub fn extract_text_from_file(&self, file: String) -> Result<String, Box<dyn Error>> {
        println!("Extracting text from file: {}", file);
        if let Some(extract) = self.map.get(&file[file.rfind('.').unwrap()..]) {
            extract(&file)
        } else {
            Err("Unsupported file type".into())
        }
    }
}