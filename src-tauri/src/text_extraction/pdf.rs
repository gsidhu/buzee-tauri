use std::{error::Error, path::Path};
use pdf_extract::extract_text;

// OCR fallback (native WinRT on Windows, textra sidecar on macOS) is opt-in via
// the `ocr` feature. The default build only performs text-layer extraction with
// pdf-extract.
#[cfg(feature = "ocr")]
use crate::housekeeping::get_app_directory;
#[cfg(all(target_os = "macos", feature = "ocr"))]
use crate::text_extraction::txt;
#[cfg(all(target_os = "macos", feature = "ocr"))]
use tauri_plugin_shell::{ShellExt, process::CommandEvent};

pub async fn extract(file: &String, _app: &tauri::AppHandle) -> Result<String, Box<dyn Error>> {
  println!("Extracting text from: {}", file);
  // check if the file contains pdf in its name
  let mut text_based_content = String::new();
  if file.to_lowercase().contains(".pdf") {
    text_based_content = match text_based_extraction(file) {
        Ok(content) => content,
        Err(_) => "false".to_string(),
    };
  }

  if text_based_content != "false" && text_based_content.len() > 0 {
    return Ok(text_based_content)
  }

  // Fallback to OCR only when the `ocr` feature is enabled.
  // With the default build, a scanned PDF without a text layer is not indexed
  // as full-text. This error is non-fatal: the scan continues and the document
  // stays searchable by name/path.
  #[cfg(feature = "ocr")]
  {
    println!("Running OCR based text extraction");
    let app_directory = get_app_directory();

    #[cfg(target_os = "macos")]
    {
      let output_path = format!("{}/temp_output.txt", app_directory);

      // run textra on the file
      let sidecar_command = _app.shell().sidecar("textra").unwrap().args([file, "-o", output_path.as_str()]);
      let (mut rx, mut _child) = sidecar_command.spawn().unwrap();

      // LOGIC:
      // textra prints only the text to stdout, everything else goes to stderr
      // but since we define an output file, there is no stdout
      // so we just poll the stderr to keep the loop running till the extraction completes
      while let Some(event) = rx.recv().await {
        if let CommandEvent::Stderr(line) = event {
          let _output_line = String::from_utf8(line).unwrap();
        }
      }

      // read the temporary file
      let temp_file_path = format!("{}/temp_output.txt", app_directory);
      let text = txt::extract(&temp_file_path, _app)?;

      // return the extracted text
      return Ok(text)
    }

    #[cfg(target_os = "windows")]
    {
      // Native Windows OCR (Windows.Media.Ocr) only handles images, not PDFs.
      // A scanned PDF without a text layer is deliberately left non-OCR'd: it
      // stays indexed by name/path but is not full-text searchable. Non-fatal,
      // so the general scan is not interrupted.
      let _ = app_directory;
      println!("Scanned PDF without text layer is not OCR-able by native Windows OCR: {}", file);
      Err("OcrUnavailableForPdf".into())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
      let _ = app_directory;
      Err("OCR sidecars are not supported on this platform".into())
    }
  }

  #[cfg(not(feature = "ocr"))]
  {
    Err("OCR is disabled in this build; the PDF has no extractable text layer".into())
  }
}

use std::panic::{catch_unwind, AssertUnwindSafe};

pub fn text_based_extraction(file: &String) -> Result<String, Box<dyn Error>> {
  let result = catch_unwind(AssertUnwindSafe(|| extract_text(Path::new(file))));

  let content = match result {
    Ok(Ok(content)) => content,
    Ok(Err(_)) | Err(_) => return Ok("false".to_string()),
  };
  Ok(content)
}
