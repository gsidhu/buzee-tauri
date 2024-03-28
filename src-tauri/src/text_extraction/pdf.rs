use std::error::Error;
use std::fs::File;

use poppler::{PopplerDocument, PopplerPage};
use cairo::{ImageSurface, Context, Format};
use tesseract::Tesseract;

pub fn extract(file: &String) -> Result<String, Box<dyn Error>> {
  // Call the extract_text_or_convert_to_png function
  let text = extract_text_or_convert_to_png(file)?;

  Ok(text)
}

fn extract_text_from_page(page: &PopplerPage) -> Option<String> {
  let text: Option<&str> = page.get_text();
  if let Some(t) = text {
      if !t.trim().is_empty() {
          return Some(t.to_string());
      }
  }
  None
}

fn convert_page_to_png(page: &PopplerPage, i: i32) -> Result<String, Box<dyn Error>> {
  let (width, height) = page.get_size();
  let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
  let context = Context::new(&surface).unwrap();

  context.save().unwrap();
  page.render(&context);
  context.restore().unwrap();
  context.show_page().unwrap();

  let png_path = format!("output_{}.png", i);
  let mut f: File = File::create(&png_path).unwrap();
  surface.write_to_png(&mut f).expect("Unable to write PNG");

  Ok(png_path)
}

fn extract_text_from_png(png_path: &str) -> Result<String, Box<dyn Error>> {
  let mut tess = Tesseract::new(None, Some("eng"))?;
  let mut tess_clone = tess.set_image(png_path)?;
  let ocr_text = tess_clone.get_text()?;
  Ok(ocr_text)
}

fn extract_text_or_convert_to_png(path: &str) -> Result<String, Box<dyn Error>> {
  let doc: PopplerDocument = PopplerDocument::new_from_file(path, None)?;
  let mut all_text: String = String::new();

  for i in 0..doc.get_n_pages() {
      let page = doc.get_page(0).unwrap();

      if let Some(text) = extract_text_from_page(&page) {
          all_text.push_str(&text);
          continue;
      }

      let png_path = convert_page_to_png(&page, i as i32)?;
      let ocr_text = extract_text_from_png(&png_path)?;
      all_text.push_str(&ocr_text);
  }

  Ok(all_text)
}
