// Native Windows OCR via the Windows Runtime (Windows.Media.Ocr).
//
// This module replaces the external `winocr` sidecar + Poppler download on
// Windows with an in-process WinRT implementation. It only ever operates on
// images (PNG/JPEG/BMP/TIFF); scanned PDFs are deliberately NOT rasterized
// here and surface as `OcrUnavailableForPdf` upstream.
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
}

// ---------------------------------------------------------------------------
// Pure helpers (platform-neutral, unit-tested)
// ---------------------------------------------------------------------------

/// Minimum number of characters below which extraction falls back to OCR.
pub const OCR_FALLBACK_MIN_CHARS: usize = 1;

/// Returns `true` when the extracted text is useful enough to skip OCR.
pub fn has_usable_text(text: &str, min_chars: usize) -> bool {
  text.trim().chars().count() >= min_chars
}

/// Returns `true` when OCR fallback should be attempted for a document whose
/// native extraction produced `extracted_text`.
pub fn should_fallback_to_ocr(extracted_text: &str, min_chars: usize) -> bool {
  !has_usable_text(extracted_text, min_chars)
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
}

#[cfg(target_os = "windows")]
mod windows_ocr {
  use super::{OcrError, OcrResult};
  use std::path::Path;
  use windows::core::HSTRING;
  use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};
  use windows::Globalization::Language;
  use windows::Graphics::Imaging::{BitmapDecoder, BitmapPixelFormat, SoftwareBitmap};
  use windows::Media::Ocr::OcrEngine;
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

  pub fn recognize(path: &Path, preferred_language: Option<&str>) -> Result<OcrResult, OcrError> {
    // WinRT requires a COM apartment on the calling thread. The caller runs us
    // on a blocking pool thread, so this never touches the UI thread.
    let init = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
    if let Err(error) = init.ok() {
      return Err(win_err(error));
    }
    let should_uninitialize = init.0 == 0;
    struct MtaGuard(bool);
    impl Drop for MtaGuard {
      fn drop(&mut self) {
        if self.0 {
          unsafe {
            CoUninitialize();
          }
        }
      }
    }
    let _guard = MtaGuard(should_uninitialize);

    let path_string = path.to_string_lossy();
    let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(path_string.as_ref()))
      .map_err(win_err)?
      .get()
      .map_err(map_file_error)?;

    let stream = file
      .OpenAsync(FileAccessMode::Read)
      .map_err(win_err)?
      .get()
      .map_err(map_file_error)?;

    let decoder = BitmapDecoder::CreateAsync(&stream)
      .map_err(|_error| OcrError::DecodeFailed)?
      .get()
      .map_err(|_error| OcrError::DecodeFailed)?;

    let native_bitmap = decoder
      .GetSoftwareBitmapAsync()
      .map_err(|_error| OcrError::DecodeFailed)?
      .get()
      .map_err(|_error| OcrError::DecodeFailed)?;

    // OCR only accepts a few pixel formats; BGRA8 premultiplied is the safest.
    let bitmap = SoftwareBitmap::Convert(&native_bitmap, BitmapPixelFormat::Bgra8)
      .map_err(|_error| OcrError::DecodeFailed)?;

    let engine = select_engine(preferred_language)?;

    let ocr = engine
      .RecognizeAsync(&bitmap)
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

    let language_tag = engine
      .RecognizerLanguage()
      .ok()
      .and_then(|language| language.LanguageTag().ok())
      .map(|tag| tag.to_string());

    Ok(OcrResult {
      text: super::normalize_ocr_text(&parts.join("\n")),
      language_tag,
      lines_detected: count,
    })
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
    assert!(!should_fallback_to_ocr("hello", OCR_FALLBACK_MIN_CHARS));
    assert!(!should_fallback_to_ocr("  hello  ", OCR_FALLBACK_MIN_CHARS));
    assert!(should_fallback_to_ocr("", OCR_FALLBACK_MIN_CHARS));
    assert!(should_fallback_to_ocr("   ", OCR_FALLBACK_MIN_CHARS));
    assert!(should_fallback_to_ocr("ab", 3));
    assert!(!has_usable_text("", 1));
    assert!(has_usable_text("e", 1));
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
