// use crate::database::crud::add_files_to_database;
use jwalk::WalkDir;
use crate::database::models::DocumentItem;
use crate::utils::get_metadata;
use std::time::UNIX_EPOCH;
// use crate::chrono;

const ALLOWED_FILETYPES: [&str; 11] = ["csv", "docx", "key", "md", "numbers", "pages", "pdf", "pptx", "txt", "xlsx", "xls"];
const FORBIDDEN_DIRECTORIES: [&str; 6] = [".git", "node_modules", "venv", "node_modules", "bower_components", "pycache"];
const WINDOWS_FORBIDDEN_DIRECTORIES: [&str; 6] = ["$RECYCLE.BIN", "System Volume Information", "AppData", "ProgramData", "Windows", "Program Files"];
const MAC_FORBIDDEN_DIRECTORIES: [&str; 6] = ["Library", "System", "bin", "usr", "sbin", "dev"];

fn get_all_forbidden_directories() -> Vec<&'static str> {
  let mut all_forbidden_directories: Vec<&str> = vec![];
  all_forbidden_directories.extend(FORBIDDEN_DIRECTORIES.iter());
  if cfg!(windows) {
    all_forbidden_directories.extend(WINDOWS_FORBIDDEN_DIRECTORIES.iter());
  } else if cfg!(mac) {
    all_forbidden_directories.extend(MAC_FORBIDDEN_DIRECTORIES.iter());
  }
  all_forbidden_directories
}

pub fn walk_directory(path: &str) {
    let mut files_array: Vec<DocumentItem> = vec![];
    let all_forbidden_directories = get_all_forbidden_directories();
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();

        // if the path contains any of the forbidden directories, continue
        if all_forbidden_directories.iter().any(|&dir| path.to_str().unwrap().contains(dir)) {
          continue;
        }
        // if the path does not contain any of the allowed filetypes, continue
        if !ALLOWED_FILETYPES.iter().any(|&dir| path.to_str().unwrap().contains(dir)) {
          continue;
        }
        // if the path does not exist or is not a file, continue
        if !path.exists() || !path.is_file() {
          continue;
        }
        
        let metadata = get_metadata(&path).unwrap();

        if metadata.is_symlink() {
          continue;
        }

        let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let mut extension = path.extension().and_then(std::ffi::OsStr::to_str);
        let is_folder = metadata.is_dir();
        if is_folder {
          extension = Some("folder");
        }
        let filesize = metadata.len();
        
        // get UNIX timestamp for last_modified, last_opened and created_at
        // and store it as text string
        let last_modified_secs = metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let created_at = metadata.created().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let last_opened = metadata.accessed().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        
        // If extension is None or is_folder is true, continue
        if extension.is_none() || is_folder {
          continue;
        }
        // If ALLOWED_FILETYPES does not contain `extension`, continue
        if !ALLOWED_FILETYPES.contains(&extension.unwrap()) {
          continue;
        }

        let file_item = DocumentItem {
          created_at: created_at.to_string(),
          name: filename.to_string(),
          path: path.to_str().unwrap().to_string(),
          size: Some(filesize as f64),
          file_type: extension.unwrap().to_string(),
          content: None,
          last_modified: last_modified_secs.to_string(),
          last_opened: last_opened.to_string(),
        };

        files_array.push(file_item);

        // if there are 10 items in cloned_files_array, add them to the database and clear the array
        if files_array.len() == 10 {
          let cloned_files_array = files_array.clone();
          add_files_to_database(cloned_files_array);
          files_array.clear();
        }
    }
}

use crate::database::schema::document::{self, last_modified};
use crate::database::establish_connection;
use diesel::RunQueryDsl; // Import the RunQueryDsl trait

pub fn add_files_to_database(files_array: Vec<DocumentItem>) {
  println!("Adding files to database: {}", files_array[0].name);
  let mut connection = establish_connection();
  let _ = diesel::insert_into(document::table)
    .values(&files_array)
    .execute(&mut connection) // Pass the connection by mutable reference
    .unwrap();
}