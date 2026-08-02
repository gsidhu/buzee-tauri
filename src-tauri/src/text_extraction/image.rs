use std::{error::Error, fs::File, io::BufReader};

// OCR fallback (native WinRT on Windows, textra sidecar on macOS) is opt-in via
// the `ocr` feature. The default build only extracts text embedded in SVG files.
#[cfg(all(target_os = "macos", feature = "ocr"))]
use crate::housekeeping::get_app_directory;
#[cfg(all(target_os = "macos", feature = "ocr"))]
use crate::text_extraction::txt;
#[cfg(all(target_os = "macos", feature = "ocr"))]
use tauri_plugin_shell::{ShellExt, process::CommandEvent};

use crate::text_extraction::win_ocr::{has_usable_text, OCR_FALLBACK_MIN_CHARS};

#[cfg(all(target_os = "windows", feature = "ocr"))]
const IMAGE_OCR_TIMEOUT_SECS: u64 = 60;

pub async fn extract(file: &String, _app: &tauri::AppHandle) -> Result<String, Box<dyn Error>> {
  // check if the file contains svg in its name
  let mut text_based_content = String::new();

  if file.to_lowercase().contains(".svg") {
    text_based_content = extract_text_from_svg(file).unwrap_or_else(|error| {
      log::error!("Failed to extract text from SVG {}: {}", file, error);
      String::new()
    });
  }

  // If the SVG already yields usable text, skip OCR entirely.
  if has_usable_text(&text_based_content, OCR_FALLBACK_MIN_CHARS) {
    return Ok(text_based_content)
  }

  // Fallback to OCR only when the `ocr` feature is enabled.
  // With the default build, images without embedded text are not indexed
  // as full-text. This error is non-fatal: the scan continues and the document
  // stays searchable by name/path.
  #[cfg(feature = "ocr")]
  {
    #[cfg(target_os = "windows")]
    {
      // Native Windows OCR (Windows.Media.Ocr). Runs on a blocking pool thread
      // so the async runtime and the UI thread are never blocked.
      use crate::text_extraction::win_ocr::{self, ImageOcr, OcrError, WindowsOcr};
      use crate::text_extraction::ocr_cache;
      use crate::database::establish_connection;

      let path = std::path::PathBuf::from(file);
      let extension = win_ocr::image_extension(&path).unwrap_or_default();
      if !win_ocr::is_supported_image(&extension) {
        log::info!("Image format not supported by Windows OCR: {}", file);
        return Err(Box::new(OcrError::UnsupportedFormat));
      }

      let mut conn = establish_connection(_app);

      // Check the OCR cache before running expensive recognition.
      let file_hash = ocr_cache::compute_file_hash(&path).unwrap_or_default();
      if let Some(cached) = ocr_cache::get_cached_ocr(&file_hash, &mut conn) {
        return Ok(cached);
      }

      let ocr = WindowsOcr;
      let result = tokio::time::timeout(
        std::time::Duration::from_secs(IMAGE_OCR_TIMEOUT_SECS),
        tokio::task::spawn_blocking(move || ocr.recognize_image(&path, None)),
      )
      .await
      .map_err(|_elapsed| {
        log::error!("Image OCR timed out after {}s: {}", IMAGE_OCR_TIMEOUT_SECS, file);
        Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "Image OCR timed out")) as Box<dyn Error>
      })?
      .map_err(|error| {
        log::error!("OCR task panicked for {}: {}", file, error);
        Box::new(error) as Box<dyn Error>
      })??;
      if result.text.trim().is_empty() {
        log::info!("OCR produced no text for image: {}", file);
      }

      // Store the result in the OCR cache for subsequent runs.
      ocr_cache::store_ocr_result(
        &file_hash,
        &result.text,
        result.lines_detected as i32,
        result.language_tag.as_deref(),
        &mut conn,
      );

      return Ok(result.text);
    }

    #[cfg(target_os = "macos")]
    {
      let app_directory = get_app_directory();
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

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
      Err("OCR sidecars are not supported on this platform".into())
    }
  }

  #[cfg(not(feature = "ocr"))]
  {
    Err("OCR is disabled in this build; the image has no extractable text".into())
  }
}

fn extract_text_from_svg(file_path: &String) -> Result<String, Box<dyn Error>> {
  use xml::reader::{EventReader, XmlEvent};
  // Open the SVG file
  let file = File::open(file_path)?;
  let file = BufReader::new(file);

  // Create an XML parser
  let parser = EventReader::new(file);

  // Iterate through the XML events
  let mut inside_text = false;
  let mut extracted_text = String::new();

  for event in parser {
      match event {
          Ok(XmlEvent::StartElement { name, .. }) => {
              if name.local_name == "text" {
                  inside_text = true;
              }
          }
          Ok(XmlEvent::Characters(data)) => {
              if inside_text {
                  extracted_text.push_str(&data);
              }
          }
          Ok(XmlEvent::EndElement { name }) => {
              if name.local_name == "text" {
                  inside_text = false;
              }
          }
          Err(e) => {
              log::error!("Error parsing SVG {}: {}", file_path, e);
              break;
          }
          _ => {}
      }
  }

  Ok(extracted_text)
}
