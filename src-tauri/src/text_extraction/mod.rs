// use std::collections::HashMap;
use std::error::Error;

pub mod csv;
pub mod docx;
pub mod epub;
pub mod mobi;
pub mod pdf;
pub mod pptx;
pub mod txt;
pub mod xlsx;
pub mod image;

pub struct Extractor;

impl Extractor {
  pub fn new() -> Self {
    Extractor
  }

  pub async fn extract_text_from_file(
    &self,
    file_path: String,
    file_type: String,
    app: &tauri::AppHandle
  ) -> Result<String, Box<dyn Error>> {
    // println!("Extracting text from file: {}", file_path);
    match file_type.as_str() {
      "csv" => csv::extract(&file_path, app),
      "docx" => docx::extract(&file_path, app),
      "epub" => epub::extract(&file_path, app),
      "mobi" => mobi::extract(&file_path, app),
      "md" => txt::extract(&file_path, app),
      "pdf" => pdf::extract(&file_path, app).await,
      "pptx" => pptx::extract(&file_path, app),
      "txt" => txt::extract(&file_path, app),
      "xlsx" => xlsx::extract(&file_path, app),
      "jpg" => image::extract(&file_path, app).await,
      "jpeg" => image::extract(&file_path, app).await,
      "png" => image::extract(&file_path, app).await,
      "svg" => image::extract(&file_path, app).await,
      _ => Err("File type not supported".into()),
    }
  }
}