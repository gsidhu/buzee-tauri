// use crate::database::crud::add_files_to_database;
use jwalk::WalkDir;
use crate::database::models::DocumentItem;
use crate::utils::get_metadata;
use crate::text_extraction::extract_text_from_file;
use std::time::UNIX_EPOCH;
// use crate::chrono;

const ALLOWED_FILETYPES: [&str; 11] = ["csv", "docx", "key", "md", "numbers", "pages", "pdf", "pptx", "txt", "xlsx", "xls"];
const FORBIDDEN_DIRECTORIES: [&str; 6] = [".git", "node_modules", "venv", "node_modules", "bower_components", "pycache"];
const WINDOWS_FORBIDDEN_DIRECTORIES: [&str; 6] = ["$RECYCLE.BIN", "System Volume Information", "AppData", "ProgramData", "Windows", "Program Files"];
// const MAC_FORBIDDEN_DIRECTORIES: [&str; 6] = ["Library", "System", "bin", "usr", "sbin", "dev"];

fn get_all_forbidden_directories() -> Vec<&'static str> {
  let mut all_forbidden_directories: Vec<&str> = vec![];
  all_forbidden_directories.extend(FORBIDDEN_DIRECTORIES.iter());
  if cfg!(windows) {
    all_forbidden_directories.extend(WINDOWS_FORBIDDEN_DIRECTORIES.iter());
  // } else if cfg!(mac) {
  //   all_forbidden_directories.extend(MAC_FORBIDDEN_DIRECTORIES.iter());
  }
  all_forbidden_directories
}

pub fn walk_directory(path: &str) -> usize {
  let mut connection = establish_connection();
  let mut files_array: Vec<DocumentItem> = vec![];
  let all_forbidden_directories = get_all_forbidden_directories();
  let mut files_added = 0;
  for entry in WalkDir::new(path) {
      let entry = entry.unwrap();
      let path = entry.path();

      // if the path contains any of the forbidden directories, continue
      if all_forbidden_directories.iter().any(|&dir| path.to_str().unwrap().contains(dir)) {
        // println!("ignoring directory");
        continue;
      }
      // if the path does not contain any of the allowed filetypes, continue
      if !ALLOWED_FILETYPES.iter().any(|&dir| path.to_str().unwrap().contains(dir)) {
        // println!("ignoring file type");
        continue;
      }
      // if the path does not exist or is not a file, continue
      if !path.exists() || !path.is_file() {
        // println!("ghost file");
        continue;
      }
      // if path starts with a dot or ~$, continue
      if path.to_str().unwrap().starts_with(".") || path.to_str().unwrap().starts_with("~$") {
        // println!("ignoring file");
        continue;
      }
      
      let metadata = get_metadata(&path).unwrap();

      if metadata.is_symlink() {
        // println!("ignoring shortcut");
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
      let mut file_content: Option<String> = None;
      if extension.unwrap() == "txt" || extension.unwrap() == "md" || extension.unwrap() == "docx" || extension.unwrap() == "pptx" {
        if !filename.contains("~$") {
          file_content = Some(extract_text_from_file(path.to_str().unwrap().to_string()));
        }
      }

      let file_item = DocumentItem {
        created_at: created_at as i64,
        name: filename.to_string(),
        path: path.to_str().unwrap().to_string(),
        size: Some(filesize as f64),
        file_type: extension.unwrap().to_string(),
        file_content: file_content,
        last_modified: last_modified_secs as i64,
        last_opened: last_opened as i64
      };

      files_array.push(file_item);

      // if there are 100 items in files_array, add them to the database and clear the array
      if files_array.len() == 100 {
        let cloned_files_array = files_array.clone();
        add_files_to_database(cloned_files_array, &mut connection);
        files_added += files_array.len();
        files_array.clear();
      }
    // process the leftover files after the loop ends
    if files_array.len() > 0 {
      let cloned_files_array = files_array.clone();
      add_files_to_database(cloned_files_array, &mut connection);
      files_added += files_array.len();
      files_array.clear();
    }
  }
  // return number of files_added
  files_added
}

use crate::database::schema::document;
use crate::database::establish_connection;
use diesel::{SqliteConnection, RunQueryDsl, QueryDsl, ExpressionMethods}; // Import the RunQueryDsl trait

pub fn add_files_to_database(files_array: Vec<DocumentItem>, connection: &mut SqliteConnection) {
  // println!("Adding files to database: {}", files_array[0].name);
  // let mut files_to_add: Vec<DocumentItem> = vec![];

  // for file in files_array {
  //   let file_exists = select(exists(document::table.filter(document::path.eq(&file.path))))
  //     .get_result(connection)
  //     .unwrap();
  //   if file_exists {
  //     handle_existing_file(file, connection);
  //   } else {
  //     files_to_add.push(file);
  //   }
  // }
  // let _ = diesel::insert_into(document::table)
  //   .values(&files_to_add)
  //   .execute(connection)
  //   .unwrap();

  let files_array_clone = files_array.clone();
  // collect all file paths from files_array
  let file_paths: Vec<_> = files_array.iter().map(|file| &file.path).collect();

  // get all existing files from the database
  let existing_files = document::table
    .select(document::path)
    .filter(document::path.eq_any(file_paths))
    .load::<String>(connection)
    .unwrap();

  // filter files do not exist in the database
  let files_to_add: Vec<_> = files_array
    .into_iter()
    .filter(|file| !existing_files.contains(&file.path))
    .collect();

  // filter files that already exist in the database
  let files_to_update: Vec<_> = files_array_clone
    .into_iter()
    .filter(|file| existing_files.contains(&file.path))
    .collect();

  // add the files to the database
  if files_to_add.len() > 0 {
    let _ = diesel::insert_into(document::table)
      .values(&files_to_add)
      .execute(connection)
      .unwrap();
  }

  // handle files to update
  if files_to_update.len() > 0 {
    handle_existing_files(files_to_update, connection);
  }

}

pub fn handle_existing_files(existing_files: Vec<DocumentItem>, connection: &mut SqliteConnection) {
  println!("Handling existing file: {}", existing_files[0].name);
  // for each file in existing_files only update the last_modified, last_opened and size fields
  for file in existing_files {
    let _ = diesel::update(document::table.filter(document::path.eq(&file.path)))
      .set((
        document::file_content.eq(&file.file_content),
        document::last_modified.eq(&file.last_modified),
        document::last_opened.eq(&file.last_opened),
        document::size.eq(&file.size),
      ))
      .execute(connection)
      .unwrap();
  }
}

// 1709728265