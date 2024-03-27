use std::fs;
use std::io;
use std::path::Path;
use tauri_plugin_global_shortcut::Modifiers;

pub fn get_metadata(path: &Path) -> io::Result<fs::Metadata> {
  // println!("Getting metadata for path: {:?}", path);
  let metadata = fs::metadata(path)?;
  Ok(metadata)
}

pub fn norm(path: &str) -> String {
  #[cfg(target_os = "windows")]
  {
    str::replace(path, "/", "\\")
  }
  
  #[cfg(target_os = "macos")]
  {
    str::replace(path, "\\", "/")
  }
}

pub fn string_to_modifiers(modifier: &str) -> Modifiers {
  match modifier {
    "ALT" => Modifiers::ALT,
    "ALT_GRAPH" => Modifiers::ALT_GRAPH,
    "CAPS_LOCK" => Modifiers::CAPS_LOCK,
    "CONTROL" => Modifiers::CONTROL,
    "FN" => Modifiers::FN,
    "FN_LOCK" => Modifiers::FN_LOCK,
    "META" => Modifiers::META,
    "NUM_LOCK" => Modifiers::NUM_LOCK,
    "SCROLL_LOCK" => Modifiers::SCROLL_LOCK,
    "SHIFT" => Modifiers::SHIFT,
    "SYMBOL" => Modifiers::SYMBOL,
    "SYMBOL_LOCK" => Modifiers::SYMBOL_LOCK,
    "HYPER" => Modifiers::HYPER,
    "SUPER" => Modifiers::SUPER,
    _ => Modifiers::empty()
  }
}