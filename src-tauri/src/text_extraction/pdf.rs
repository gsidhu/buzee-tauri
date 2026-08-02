use std::{error::Error, path::Path};
use pdf_extract::extract_text;

use crate::text_extraction::win_ocr::{OCR_FALLBACK_MIN_CHARS, has_usable_text};

// OCR fallback (native WinRT on Windows, textra sidecar on macOS) is opt-in via
// the `ocr` feature. The default build only performs text-layer extraction with
// pdf-extract.
#[cfg(feature = "ocr")]
use crate::housekeeping::get_app_directory;
#[cfg(all(target_os = "windows", feature = "ocr"))]
use crate::user_prefs::get_pdf_max_ocr_pages;
#[cfg(all(target_os = "macos", feature = "ocr"))]
use crate::text_extraction::txt;
#[cfg(all(target_os = "macos", feature = "ocr"))]
use tauri_plugin_shell::{ShellExt, process::CommandEvent};

/// Upper bound on how long a single PDF OCR pass may take before it is aborted.
/// A corrupt or pathological file should not pin a blocking pool thread forever.
const PDF_OCR_TIMEOUT_SECS: u64 = 120;

pub async fn extract(file: &String, _app: &tauri::AppHandle) -> Result<String, Box<dyn Error>> {
  log::info!("Extracting text from: {}", file);
  // check if the file contains pdf in its name
  let text_based_content = if file.to_lowercase().contains(".pdf") {
    // On extraction failure, treat the layer as empty so we fall through to OCR.
    text_based_extraction(file).unwrap_or_default()
  } else {
    String::new()
  };

  // If the native text layer already yields usable text, skip OCR entirely.
  if has_usable_text(&text_based_content, OCR_FALLBACK_MIN_CHARS) {
    return Ok(text_based_content)
  }

  // Fallback to OCR only when the `ocr` feature is enabled.
  // With the default build, a scanned PDF without a text layer is not indexed
  // as full-text. This error is non-fatal: the scan continues and the document
  // stays searchable by name/path.
  #[cfg(feature = "ocr")]
  {
    log::info!("Running OCR based text extraction");
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
      let _ = app_directory;
      // Native Windows OCR rasterizes scanned PDFs page by page with
      // Windows.Data.Pdf and OCRs each page with Windows.Media.Ocr. The blocking
      // WinRT calls run on a dedicated blocking thread so the async runtime and
      // the UI thread are never blocked.
      use crate::text_extraction::win_ocr::{self, ImageOcr};
      use crate::text_extraction::ocr_cache;
      use crate::database::establish_connection;

      let mut conn = establish_connection(_app);

      // Check the OCR cache before running expensive recognition.
      let file_hash = ocr_cache::compute_file_hash(std::path::Path::new(file))
        .unwrap_or_default();
      if let Some(cached) = ocr_cache::get_cached_ocr(&file_hash, &mut conn) {
        return Ok(cached);
      }

      let path_buf = std::path::PathBuf::from(file);
      let max_pages = get_pdf_max_ocr_pages(_app) as u32;
      let result = tokio::time::timeout(
        std::time::Duration::from_secs(PDF_OCR_TIMEOUT_SECS),
        tokio::task::spawn_blocking(move || {
          let ocr_engine = win_ocr::WindowsOcr;
          ocr_engine.recognize_pdf(&path_buf, None, max_pages)
        }),
      )
      .await
      .map_err(|_elapsed| {
        log::error!("PDF OCR timed out after {}s: {}", PDF_OCR_TIMEOUT_SECS, file);
        Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "PDF OCR timed out")) as Box<dyn Error>
      })?
      .map_err(|error| Box::new(error) as Box<dyn Error>)??;

      if result.text.trim().is_empty() {
        return Err("OcrUnavailableForPdf".into());
      }

      // Store the result in the OCR cache for subsequent runs.
      ocr_cache::store_ocr_result(
        &file_hash,
        &result.text,
        result.lines_detected as i32,
        result.language_tag.as_deref(),
        &mut conn,
      );

      return Ok(result.text)
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
    Ok(Err(error)) => return Err(error.into()),
    Err(_) => return Err("pdf text extraction panicked".into()),
  };
  Ok(content)
}
