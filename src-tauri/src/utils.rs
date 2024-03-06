// use std::fmt::{self, Debug};

// struct DateTime {
//   start_date: String,
//   end_date: String,
//   text: String
// }

// impl DateTime {
//   fn new(name: &str) -> Self {
//     DateTime {
//       start_date: start_date.to_string(),
//       end_date: end_date.to_string(),
//       text: text.to_string(),
//     }
//   }
// }
use std::fs;
use std::io;
use std::path::Path;

pub fn get_metadata(path: &Path) -> io::Result<fs::Metadata> {
  println!("Getting metadata for path: {:?}", path);
  let metadata = fs::metadata(path)?;
  Ok(metadata)
}

pub fn list_dir_contents(str: &str) -> Vec<String> {
  if let Ok(paths) = std::fs::read_dir(str) {
    return paths
      .into_iter()
      .map(|x| x.unwrap().path().to_str().unwrap().to_string())
      .collect();
  }
  vec![]
}

#[cfg(windows)]
pub unsafe fn get_win32_ready_drives() -> Vec<String> {
  let mut logical_drives = Vec::with_capacity(5);
  let mut bitfield = kernel32::GetLogicalDrives();
  let mut drive = 'A';

  while bitfield != 0 {
    if bitfield & 1 == 1 {
      let strfulldl = drive.to_string() + ":/";
      let cstrfulldl = CString::new(strfulldl.clone()).unwrap();
      let x = kernel32::GetDriveTypeA(cstrfulldl.as_ptr());
      if x == 3 || x == 2 {
        logical_drives.push(strfulldl);
        // println!("drive {0} is {1}", strfdl, x);
      }
    }
    drive = std::char::from_u32((drive as u32) + 1).unwrap();
    bitfield >>= 1;
  }
  logical_drives.sort_by(|x1, x2| x2.cmp(x1));
  logical_drives
}