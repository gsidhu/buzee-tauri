// Native Windows OCR via the Windows Runtime (Windows.Media.Ocr).
//
// This module replaces the external `winocr` sidecar + Poppler download on
// Windows with an in-process WinRT implementation. It recognizes images
// (PNG/JPEG/BMP/TIFF) directly and rasterizes scanned PDFs page by page with
// Windows.Data.Pdf before OCR-ing each page.
//
// The WinRT code is compiled only on Windows. The public types, the trait and
// the pure helpers are platform-neutral so routing and text normalization stay
// unit-testable everywhere.

use std::path::Path;

// ---------------------------------------------------------------------------
// Public domain types (platform-neutral)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OcrError {
  UnsupportedFormat,
  EngineUnavailable,
  LanguageUnavailable,
  DecodeFailed,
  FileUnavailable,
  AccessDenied,
  WindowsApi(String),
}

impl std::fmt::Display for OcrError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      OcrError::UnsupportedFormat => write!(f, "image format is not supported by Windows OCR"),
      OcrError::EngineUnavailable => write!(f, "no Windows OCR engine is available"),
      OcrError::LanguageUnavailable => write!(f, "requested OCR language is not installed"),
      OcrError::DecodeFailed => write!(f, "failed to decode the image"),
      OcrError::FileUnavailable => write!(f, "file is not accessible or was removed"),
      OcrError::AccessDenied => write!(f, "access to the file was denied"),
      OcrError::WindowsApi(message) => write!(f, "Windows OCR API error: {}", message),
    }
  }
}

impl std::error::Error for OcrError {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OcrResult {
  pub text: String,
  pub language_tag: Option<String>,
  pub lines_detected: usize,
}

pub trait ImageOcr {
  fn recognize_image(
    &self,
    path: &Path,
    preferred_language: Option<&str>,
  ) -> Result<OcrResult, OcrError>;

  fn recognize_pdf(
    &self,
    path: &Path,
    preferred_language: Option<&str>,
    max_pages: u32,
  ) -> Result<OcrResult, OcrError>;
}

// ---------------------------------------------------------------------------
// Pure helpers (platform-neutral, unit-tested)
// ---------------------------------------------------------------------------

/// Minimum number of characters below which extraction falls back to OCR.
/// Set high enough to avoid treating a few residual characters from a corrupt
/// text layer as "usable" and skipping OCR for an otherwise scanned document.
pub const OCR_FALLBACK_MIN_CHARS: usize = 100;

/// Returns `true` when the extracted text is useful enough to skip OCR.
pub fn has_usable_text(text: &str, min_chars: usize) -> bool {
  text.trim().chars().count() >= min_chars
}

/// Returns `true` when OCR fallback should be attempted for a document whose
/// native extraction produced `extracted_text`.
pub fn should_fallback_to_ocr(extracted_text: &str, min_chars: usize) -> bool {
  !has_usable_text(extracted_text, min_chars)
}

/// Clamps a PDF page count to an upper bound.
pub fn cap_page_count(page_count: u32, max: u32) -> u32 {
  page_count.min(max)
}

/// Lowercased file extension, without the leading dot.
pub fn image_extension(path: &Path) -> Option<String> {
  path
    .extension()
    .and_then(|extension| extension.to_str())
    .map(|extension| extension.to_lowercase())
}

/// Formats the Windows OCR engine can realistically decode via WIC.
pub fn is_supported_image(extension: &str) -> bool {
  matches!(extension, "png" | "jpg" | "jpeg" | "bmp" | "tif" | "tiff")
}

/// Minimal normalization: trims every line, collapses runs of blank lines into
/// a single one and trims the whole result. Useful line breaks are preserved.
pub fn normalize_ocr_text(raw: &str) -> String {
  let mut out = String::new();
  let mut previous_blank = false;
  for line in raw.lines() {
    let trimmed = line.trim();
    if trimmed.is_empty() {
      if !previous_blank {
        out.push('\n');
      }
      previous_blank = true;
    } else {
      out.push_str(trimmed);
      out.push('\n');
      previous_blank = false;
    }
  }
  out.trim().to_string()
}

// ---------------------------------------------------------------------------
// Windows Runtime implementation (Windows only)
// ---------------------------------------------------------------------------

#[cfg(target_os = "windows")]
pub struct WindowsOcr;

#[cfg(target_os = "windows")]
impl ImageOcr for WindowsOcr {
  fn recognize_image(
    &self,
    path: &Path,
    preferred_language: Option<&str>,
  ) -> Result<OcrResult, OcrError> {
    windows_ocr::recognize(path, preferred_language)
  }

  fn recognize_pdf(
    &self,
    path: &Path,
    preferred_language: Option<&str>,
    max_pages: u32,
  ) -> Result<OcrResult, OcrError> {
    windows_ocr::recognize_pdf(path, preferred_language, max_pages)
  }
}

// Non-Windows stub so the rest of the crate keeps compiling.
#[cfg(not(target_os = "windows"))]
pub struct WindowsOcr;

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
impl ImageOcr for WindowsOcr {
  fn recognize_image(
    &self,
    _path: &Path,
    _preferred_language: Option<&str>,
  ) -> Result<OcrResult, OcrError> {
    Err(OcrError::EngineUnavailable)
  }

  fn recognize_pdf(
    &self,
    _path: &Path,
    _preferred_language: Option<&str>,
    _max_pages: u32,
  ) -> Result<OcrResult, OcrError> {
    Err(OcrError::EngineUnavailable)
  }
}

#[cfg(target_os = "windows")]
mod windows_ocr {
  use super::{OcrError, OcrResult};
  use std::path::Path;
  use windows::core::{HSTRING, Interface};
  use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
  use windows::Data::Pdf::{PdfDocument, PdfPage};
  use windows::Globalization::Language;
  use windows::Graphics::Imaging::{BitmapDecoder, BitmapPixelFormat, SoftwareBitmap};
  use windows::Media::Ocr::OcrEngine;
  use windows::Storage::Streams::{InMemoryRandomAccessStream, IRandomAccessStream};
  use windows::Storage::{FileAccessMode, StorageFile};

  fn win_err(error: windows::core::Error) -> OcrError {
    OcrError::WindowsApi(error.to_string())
  }

  // Maps the low 16 bits of the HRESULT back to the Win32 error code for the
  // well-known cases; anything else is kept as a contextualized API error.
  pub(super) fn map_file_error(error: windows::core::Error) -> OcrError {
    match error.code().0 & 0xFFFF {
      0x0002 | 0x0003 => OcrError::FileUnavailable,
      0x0005 => OcrError::AccessDenied,
      _ => win_err(error),
    }
  }

  // COM apartment guard. WinRT requires an initialized COM apartment on the
  // calling thread; the caller runs us on blocking pool threads, so this never
  // touches the UI thread. Because pool threads are long-lived and reused,
  // COM is initialized lazily ONCE per thread (via `thread_local!`) instead of
  // being torn down on every file, which removes the repeated init/teardown
  // overhead without leaking apartments.
  thread_local! {
    // `true` once this thread has joined the MTA.
    static COM_MTA_JOINED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
  }

  struct MtaGuard;
  impl MtaGuard {
    fn init() -> Result<Self, OcrError> {
      if !COM_MTA_JOINED.with(|joined| joined.get()) {
        let init = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
        init.ok().map_err(win_err)?;
        // S_OK(0) means this thread now joined the MTA; S_FALSE(1) means it was
        // already in one (e.g. inherited). Only guard the teardown when we
        // actually joined, so we never over-uninitialize.
        if init.0 == 0 {
          COM_MTA_JOINED.with(|joined| joined.set(true));
        }
      }
      Ok(MtaGuard)
    }
  }

  fn open_readable_stream(path: &Path) -> Result<IRandomAccessStream, OcrError> {
    let path_string = path.to_string_lossy();
    let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(path_string.as_ref()))
      .map_err(win_err)?
      .get()
      .map_err(map_file_error)?;

    file
      .OpenAsync(FileAccessMode::Read)
      .map_err(win_err)?
      .get()
      .map_err(map_file_error)
  }

  // OCR is only allowed on a few pixel formats; BGRA8 premultiplied is safest.
  fn to_bgra8(stream: &IRandomAccessStream) -> Result<SoftwareBitmap, OcrError> {
    let decoder = BitmapDecoder::CreateAsync(stream)
      .map_err(|_error| OcrError::DecodeFailed)?
      .get()
      .map_err(|_error| OcrError::DecodeFailed)?;

    let native_bitmap = decoder
      .GetSoftwareBitmapAsync()
      .map_err(|_error| OcrError::DecodeFailed)?
      .get()
      .map_err(|_error| OcrError::DecodeFailed)?;

    SoftwareBitmap::Convert(&native_bitmap, BitmapPixelFormat::Bgra8)
      .map_err(|_error| OcrError::DecodeFailed)
  }

  // Runs the OCR engine on a single bitmap and returns (joined text, line count).
  fn recognize_bitmap(engine: &OcrEngine, bitmap: &SoftwareBitmap) -> Result<(String, usize), OcrError> {
    let ocr = engine
      .RecognizeAsync(bitmap)
      .map_err(win_err)?
      .get()
      .map_err(win_err)?;
    let lines = ocr.Lines().map_err(win_err)?;
    let count = lines.Size().map_err(win_err)? as usize;

    let mut parts = Vec::with_capacity(count);
    for index in 0..count {
      let line = lines.GetAt(index as u32).map_err(win_err)?;
      parts.push(line.Text().map_err(win_err)?.to_string());
    }
    Ok((parts.join("\n"), count))
  }

  pub fn recognize(path: &Path, preferred_language: Option<&str>) -> Result<OcrResult, OcrError> {
    let _guard = MtaGuard::init()?;
    let stream = open_readable_stream(path)?;
    let bitmap = to_bgra8(&stream)?;
    let engine = select_engine(preferred_language)?;
    let (text, lines_detected) = recognize_bitmap(&engine, &bitmap)?;

    Ok(OcrResult {
      text: super::normalize_ocr_text(&text),
      language_tag: engine_language_tag(&engine),
      lines_detected,
    })
  }

  pub fn recognize_pdf(path: &Path, preferred_language: Option<&str>, max_pages: u32) -> Result<OcrResult, OcrError> {
    let _guard = MtaGuard::init()?;
    let stream = open_readable_stream(path)?;

    let document = PdfDocument::LoadFromStreamAsync(&stream)
      .map_err(win_err)?
      .get()
      .map_err(win_err)?;
    let total_pages = document.PageCount().map_err(win_err)?;
    let pages_to_ocr = super::cap_page_count(total_pages, max_pages);

    if pages_to_ocr < total_pages {
      log::info!(
        "PDF {} has {} pages; OCR will only process the first {} pages",
        path.display(),
        total_pages,
        pages_to_ocr
      );
    }

    // Small PDFs: sequential (avoids spawn overhead).
    let engine = select_engine(preferred_language)?;
    if pages_to_ocr <= 3 {
      let mut page_texts = Vec::with_capacity(pages_to_ocr as usize);
      let mut lines_detected = 0usize;
      let mut first_error: Option<OcrError> = None;

      for index in 0..pages_to_ocr {
        match ocr_page(&document, &engine, index) {
          Ok((text, count)) => {
            page_texts.push(text);
            lines_detected += count;
          }
          Err(error) => {
            log::warn!("OCR failed for page {} of {}: {}", index + 1, path.display(), error);
            if first_error.is_none() {
              first_error = Some(error);
            }
          }
        }
      }

      if page_texts.is_empty() {
        return Err(first_error.unwrap_or(OcrError::DecodeFailed));
      }

      return Ok(OcrResult {
        text: super::normalize_ocr_text(&page_texts.join("\n\n")),
        language_tag: engine_language_tag(&engine),
        lines_detected,
      });
    }

    // Large PDFs: parallel OCR across pages, in batches.
    // Each spawn_blocking batch task gets its own COM apartment, PdfDocument,
    // and OcrEngine : no WinRT objects cross thread boundaries. The semaphore
    // caps how many batches (and thus how many threads) run at once, and
    // futures::join_all waits for all results. Page order is preserved because
    // results are collected in the order the batches were spawned.
    recognize_pdf_parallel(path, preferred_language, pages_to_ocr)
  }

  // Maximum number of PDF pages to OCR concurrently. Each parallel task loads
  // the PDF and creates its own OcrEngine, so this bounds peak memory.
  const MAX_PARALLEL_OCR_PAGES: usize = 8;

  // OCRs `pages_to_ocr` PDF pages in small batches, each batch run on its own
  // blocking-pool thread through `join_all`.
  fn recognize_pdf_parallel(path: &Path, preferred_language: Option<&str>, pages_to_ocr: u32) -> Result<OcrResult, OcrError> {
    use futures::future::join_all;
    use tokio::sync::Semaphore;

    let semaphore = std::sync::Arc::new(Semaphore::new(MAX_PARALLEL_OCR_PAGES));
    let preferred = preferred_language.map(|s| s.to_string());
    let path_owned = path.to_path_buf();
    let rt = tokio::runtime::Handle::current();

    // Group pages into contiguous batches. Smaller PDFs get a single batch that
    // runs sequentially on one thread; large PDFs fan out across many batches.
    let batch_size = MAX_PARALLEL_OCR_PAGES as u32;
    let batch_count = pages_to_ocr.div_ceil(batch_size);
    let mut batches: Vec<(u32, u32)> = Vec::with_capacity(batch_count as usize);
    for batch_index in 0..batch_count {
      let start = batch_index * batch_size;
      let end = (start + batch_size).min(pages_to_ocr);
      batches.push((start, end));
    }

    // Spawn one blocking task per batch. The permit is acquired here (before
    // spawning) so we never hold more than a bounded number of concurrent OCR
    // tasks, keeping the blocking pool from being saturated.
    let mut handles: Vec<tokio::task::JoinHandle<Result<(String, usize), OcrError>>> =
      Vec::with_capacity(batch_count as usize);

    for (start, end) in batches {
      let permit = rt
        .block_on(semaphore.clone().acquire_owned())
        .map_err(|_| OcrError::WindowsApi("OCR semaphore closed".into()))?;
      let pref = preferred.clone();
      let p = path_owned.clone();

      handles.push(tokio::task::spawn_blocking(move || {
        let _permit = permit; // held until the batch completes
        ocr_page_batch_task(&p, pref.as_deref(), start, end)
      }));
    }

    // We are on a blocking pool thread (caller wrapped us in spawn_blocking), so
    // blocking on join_all is safe and expected; it waits for every batch.
    let batch_results: Vec<Result<Result<(String, usize), OcrError>, tokio::task::JoinError>> =
      rt.block_on(join_all(handles));

    let mut page_texts = Vec::with_capacity(pages_to_ocr as usize);
    let mut lines_detected = 0usize;
    let mut first_error: Option<OcrError> = None;

    for result in batch_results {
      match result {
        Ok(Ok((text, count))) => {
          page_texts.push(text);
          lines_detected += count;
        }
        Ok(Err(error)) => {
          log::warn!("OCR batch failed for {}: {}", path.display(), error);
          if first_error.is_none() {
            first_error = Some(error);
          }
        }
        Err(join_error) => {
          log::warn!("OCR batch task panicked for {}: {}", path.display(), join_error);
          if first_error.is_none() {
            first_error = Some(OcrError::WindowsApi("batch task panicked".into()));
          }
        }
      }
    }

    if page_texts.is_empty() {
      return Err(first_error.unwrap_or(OcrError::DecodeFailed));
    }

    Ok(OcrResult {
      text: super::normalize_ocr_text(&page_texts.join("\n\n")),
      language_tag: None, // per-page engine; language already logged
      lines_detected,
    })
  }

  // OCRs a contiguous page range [start, end) sequentially on the calling
  // thread, using its own COM handle, PdfDocument and OcrEngine. Safe to call
  // from any thread : no WinRT objects are shared.
  fn ocr_page_batch_task(
    path: &Path,
    preferred_language: Option<&str>,
    start: u32,
    end: u32,
  ) -> Result<(String, usize), OcrError> {
    let _guard = MtaGuard::init()?;
    let stream = open_readable_stream(path)?;
    let document = PdfDocument::LoadFromStreamAsync(&stream)
      .map_err(win_err)?
      .get()
      .map_err(win_err)?;
    let engine = select_engine(preferred_language)?;

    let mut page_texts = Vec::with_capacity((end - start) as usize);
    let mut lines_detected = 0usize;
    let mut first_error: Option<OcrError> = None;

    for index in start..end {
      match ocr_page(&document, &engine, index) {
        Ok((text, count)) => {
          page_texts.push(text);
          lines_detected += count;
        }
        Err(error) => {
          log::warn!("OCR failed for page {} of {}: {}", index + 1, path.display(), error);
          if first_error.is_none() {
            first_error = Some(error);
          }
        }
      }
    }

    if page_texts.is_empty() {
      return Err(first_error.unwrap_or(OcrError::DecodeFailed));
    }

    Ok((page_texts.join("\n\n"), lines_detected))
  }

  // Rasterizes a single PDF page into a bitmap and OCRs it.
  fn ocr_page(document: &PdfDocument, engine: &OcrEngine, index: u32) -> Result<(String, usize), OcrError> {
    let page: PdfPage = document.GetPage(index).map_err(win_err)?;
    let output = InMemoryRandomAccessStream::new().map_err(win_err)?;
    page
      .RenderToStreamAsync(&output)
      .map_err(win_err)?
      .get()
      .map_err(win_err)?;
    output.Seek(0).map_err(win_err)?;

    let stream = output.cast::<IRandomAccessStream>().map_err(win_err)?;
    let bitmap = to_bgra8(&stream)?;
    recognize_bitmap(engine, &bitmap)
  }

  fn engine_language_tag(engine: &OcrEngine) -> Option<String> {
    engine
      .RecognizerLanguage()
      .ok()
      .and_then(|language| language.LanguageTag().ok())
      .map(|tag| tag.to_string())
  }

  fn select_engine(preferred_language: Option<&str>) -> Result<OcrEngine, OcrError> {
    if let Some(tag) = preferred_language {
      // An explicit preference that is not installed is reported instead of
      // silently picking a different language.
      let language =
        Language::CreateLanguage(&HSTRING::from(tag)).map_err(|_| OcrError::LanguageUnavailable)?;
      let requested = language.LanguageTag().map_err(win_err)?.to_string();
      let available = OcrEngine::AvailableRecognizerLanguages().map_err(win_err)?;
      let size = available.Size().map_err(win_err)?;

      let mut found = false;
      for index in 0..size {
        let candidate = available.GetAt(index).map_err(win_err)?;
        if candidate.LanguageTag().map_err(win_err)?.to_string() == requested {
          found = true;
          break;
        }
      }
      if !found {
        return Err(OcrError::LanguageUnavailable);
      }
      OcrEngine::TryCreateFromLanguage(&language).map_err(|_| OcrError::EngineUnavailable)
    } else {
      // Any engine matching the user profile languages; NULL -> no engine.
      OcrEngine::TryCreateFromUserProfileLanguages().map_err(|_| OcrError::EngineUnavailable)
    }
  }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn supported_image_extensions() {
    assert!(is_supported_image("png"));
    assert!(is_supported_image("jpg"));
    assert!(is_supported_image("jpeg"));
    assert!(is_supported_image("bmp"));
    assert!(is_supported_image("tif"));
    assert!(is_supported_image("tiff"));
    assert!(!is_supported_image("gif"));
    assert!(!is_supported_image("pdf"));
    assert!(!is_supported_image(""));
  }

  #[test]
  fn extension_is_lowercased_without_dot() {
    assert_eq!(image_extension(Path::new("photo.PNG")), Some("png".to_string()));
    assert_eq!(image_extension(Path::new("doc.jpg")), Some("jpg".to_string()));
    assert_eq!(image_extension(Path::new("archive.tar.gz")), Some("gz".to_string()));
    assert_eq!(image_extension(Path::new("no_extension")), None);
  }

  #[test]
  fn ocr_fallback_threshold() {
    // "hello" is far below the 100-char threshold, so OCR fallback IS expected.
    assert!(should_fallback_to_ocr("hello", OCR_FALLBACK_MIN_CHARS));
    assert!(should_fallback_to_ocr("  hello  ", OCR_FALLBACK_MIN_CHARS));
    assert!(!has_usable_text("hello", OCR_FALLBACK_MIN_CHARS));
    assert!(should_fallback_to_ocr("", OCR_FALLBACK_MIN_CHARS));
    assert!(should_fallback_to_ocr("   ", OCR_FALLBACK_MIN_CHARS));
    assert!(should_fallback_to_ocr("ab", 3));
    assert!(!should_fallback_to_ocr("abcd", 3));
    assert!(!has_usable_text("", 1));
    assert!(has_usable_text("e", 1));
    assert!(has_usable_text(&"x".repeat(100), OCR_FALLBACK_MIN_CHARS));
  }

  #[test]
  fn normalize_keeps_lines_and_trims() {
    let input = "  Hello world   \n\n\n\nSecond line\n   \nThird line  ";
    assert_eq!(
      normalize_ocr_text(input),
      "Hello world\n\nSecond line\n\nThird line"
    );
  }

  #[test]
  fn normalize_empty_and_single_line() {
    assert_eq!(normalize_ocr_text(""), "");
    assert_eq!(normalize_ocr_text("   \n  \n"), "");
    assert_eq!(normalize_ocr_text("  only line  "), "only line");
  }

  #[test]
  fn normalize_preserves_inner_blanks_single() {
    assert_eq!(normalize_ocr_text("a\n\n\n\nb"), "a\n\nb");
  }

  #[test]
  fn pdf_page_cap_is_bounded() {
    assert_eq!(cap_page_count(0, 150), 0);
    assert_eq!(cap_page_count(1, 150), 1);
    assert_eq!(cap_page_count(10, 150), 10);
    assert_eq!(cap_page_count(150, 150), 150);
    assert_eq!(cap_page_count(200, 150), 150);
  }

  #[cfg(target_os = "windows")]
  #[test]
  fn file_error_mapping_is_business_typed() {
    use windows::core::{Error, HRESULT};
    assert_eq!(
      windows_ocr::map_file_error(Error::from_hresult(HRESULT(-2147024894))),
      OcrError::FileUnavailable
    );
    assert_eq!(
      windows_ocr::map_file_error(Error::from_hresult(HRESULT(-2147024891))),
      OcrError::AccessDenied
    );
    assert!(matches!(
      windows_ocr::map_file_error(Error::from_hresult(HRESULT(-2147467263))),
      OcrError::WindowsApi(_)
    ));
  }
}
