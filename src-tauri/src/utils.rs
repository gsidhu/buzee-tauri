use std::fs;
use std::io;
use std::path::Path;

pub fn get_metadata(path: &Path) -> io::Result<fs::Metadata> {
  // println!("Getting metadata for path: {:?}", path);
  let metadata = fs::metadata(path)?;
  Ok(metadata)
}