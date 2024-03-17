// use crate::database::crud::add_files_to_database;
use jwalk::{WalkDir, WalkDirGeneric};
use crate::{database::models::DocumentItem, text_extraction::Extractor};
use crate::utils::{self, get_metadata};
use std::time::UNIX_EPOCH;
use crate::housekeeping::get_home_directory;
use diesel::connection::Connection;
use crate::database::schema::document;
use crate::database::establish_connection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use log::{info, trace, warn, error};

const DOCUMENT_FILETYPES: [&str; 11] = ["csv", "docx", "key", "md", "numbers", "pages", "pdf", "pptx", "txt", "xlsx", "xls"];
const IMAGE_FILETYPES: [&str; 5] = ["jpg", "jpeg", "png", "gif", "svg"];
const BOOK_FILETYPES: [&str; 3] = ["epub", "mobi", "azw3"];
const AUDIO_FILETYPES: [&str; 5] = ["mp3", "wav", "aac", "flac", "ogg"];
const VIDEO_FILETYPES: [&str; 5] = ["mp4", "mkv", "avi", "mov", "wmv"];

pub fn all_allowed_filetypes() -> Vec<String> {
  let mut allowed_filetypes: Vec<String> = vec![];
  allowed_filetypes.extend(DOCUMENT_FILETYPES.iter().map(|&s| s.to_string()));
  allowed_filetypes.extend(IMAGE_FILETYPES.iter().map(|&s| s.to_string()));
  allowed_filetypes.extend(BOOK_FILETYPES.iter().map(|&s| s.to_string()));
  allowed_filetypes.extend(AUDIO_FILETYPES.iter().map(|&s| s.to_string()));
  allowed_filetypes.extend(VIDEO_FILETYPES.iter().map(|&s| s.to_string()));
  allowed_filetypes
}

fn get_all_forbidden_directories() -> Vec<String> {
  let home_dir: String = get_home_directory().unwrap();
  let mut all_forbidden_directories: Vec<String> = vec![];
  let forbidden_directories: [&str; 4] = ["node_modules", "venv", "bower_components", "pycache"];
  all_forbidden_directories.extend(forbidden_directories.iter().map(|&s| s.to_string()));
  #[cfg(target_os = "windows")]
  {
    let windows_forbidden_directories: [&str; 6] = ["$RECYCLE.BIN", "System Volume Information", "AppData", "ProgramData", "Windows", "Program Files"];
    all_forbidden_directories.extend(windows_forbidden_directories.iter().map(|&s| s.to_string()));
  }
  #[cfg(target_os = "macos")]
  {
    let mac_forbidden_directories: [&str; 2] = [&format!("{}/Library", home_dir), &format!("{}/Applications", home_dir)];
    all_forbidden_directories.extend(mac_forbidden_directories.iter().map(|&s| s.to_string()));
  }
  println!("Forbidden directories: {:?}", all_forbidden_directories);
  all_forbidden_directories
}

fn build_walk_dir(path: &String, skip_path: Vec<String>) -> WalkDirGeneric<((), ())> {
  WalkDir::new(path).process_read_dir(move |_, _, _, children| {
    children.iter_mut().for_each(|dir_entry_result| {
      if let Ok(dir_entry) = dir_entry_result {
        let curr_path = utils::norm(dir_entry.path().to_str().unwrap_or(""));

        // let guard = USER_SETTING.read().unwrap();
        // let exclude_path = guard.exclude_index_path();

        // if exclude_path.iter().any(|x| curr_path.starts_with(x))
        if skip_path.iter().any(|x| curr_path.contains(x)) {
          info!("skip path {}", curr_path);
          dir_entry.read_children_path = None;
        }
      }
    });
  })
}

pub fn walk_directory(path: &str) -> usize {
  info!("Logger initialized!");
  let mut connection = establish_connection();
  let mut files_array: Vec<DocumentItem> = vec![];
  let all_forbidden_directories = get_all_forbidden_directories();
  let mut files_added = 0;
  let allowed_filetypes = all_allowed_filetypes();

  let walk_dir = build_walk_dir(&path.to_string(), all_forbidden_directories);

  // for entry in WalkDir::new(path) {
  for entry in walk_dir {
      let entry = entry.unwrap();
      let path = entry.path();
      // info!("Indexing: {}", path.to_str().unwrap());
      
      // if the path does not exist or is not a file, continue
      if !path.exists() || !path.is_file() {
        println!("Folder maybe?: {}", path.to_str().unwrap());
        continue;
      }

      let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
      let mut extension = path.extension().and_then(|s| s.to_str());

      // if extension is not in allowed filetypes, continue
      if extension.is_none() || !allowed_filetypes.contains(&extension.unwrap().to_string()) {
        // println!("ignoring file");
        continue;
      }
      // if filename starts with a dot or ~$, continue
      if filename.starts_with(".") || filename.starts_with("~$") {
        // println!("ignoring file");
        continue;
      }
      
      let metadata = get_metadata(&path).unwrap();
      // if metadata is a symlink or shortcut file, continue
      if metadata.file_type().is_symlink() {
        // println!("ignoring shortcut");
        continue;
      }

      let is_folder = metadata.is_dir();
      if is_folder {
        extension = Some("folder");
      }
      let filesize = metadata.len();
      
      // get UNIX timestamp for last_modified, last_opened and created_at and store it as text string
      let last_modified_secs = metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
      let created_at = metadata.created().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
      let last_opened = metadata.accessed().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
      
      // If extension is None or is_folder is true, continue
      if extension.is_none() || is_folder {
        continue;
      }
      // If ALLOWED_FILETYPES does not contain `extension`, continue
      // let mut file_content: Option<String> = None;
      // if extension.unwrap() == "txt" || extension.unwrap() == "md" || extension.unwrap() == "docx" || extension.unwrap() == "pptx" {
      //   if !filename.contains("~$") || !filename.contains(".tmp") || !filename.contains(".temp")  {
      //     file_content = Some(extract_text_from_file(path.to_str().unwrap().to_string()));
      //   }
      // }

      let file_item = DocumentItem {
        source_domain: "local".to_string(),
        created_at: created_at as i64,
        name: filename.to_string(),
        path: path.to_str().unwrap().to_string(),
        size: Some(filesize as f64),
        file_type: extension.unwrap().to_string(),
        last_modified: last_modified_secs as i64,
        last_opened: last_opened as i64,
        last_synced: 0,
        is_pinned: false,
        frecency_rank: 0.0,
        frecency_last_accessed: 0,
        comment: None,
      };

      files_array.push(file_item);

      // if there are 500 items in metadata_array, add them to the database and clear the array
      if files_array.len() == 500 {
        add_metadata_to_database(&files_array, &mut connection);
        files_added += files_array.len();
        files_array.clear();
      }
    // process the leftover files after the loop ends
    if files_array.len() > 0 {
      // let cloned_files_array = files_array.clone();
      add_metadata_to_database(&files_array, &mut connection);
      files_added += files_array.len();
      files_array.clear();
    }
  }
  // return number of files_added
  files_added
}

// pub fn parse_content_from_file(path: &str) -> usize {
//   // get file paths, extensions, file_content, last_modified, last_opened from all rows in the database
//   let files_data = document::table
//     .select((document::path, document::file_type, document::file_content, document::last_modified, document::last_opened))
//     .load::<(String, String, Option<String>, i64, i64)>(connection)
//     .unwrap();
//   let extractor: Extractor = Extractor::new();
//   for (path, file_type, file_content, last_modified, last_opened) in files_data {
//     // for each file, if the extension is in DOCUMENT_FILETYPES
//     // extract the text from path and update file_content
//     if DOCUMENT_FILETYPES.contains(&file_type.as_str()) {
//       let extracted_text = extractor.extract_text_from_file(path);
//       let extracted_text = match extracted_text {
//         Ok(text) => text,
//         Err(e) => {
//           eprintln!("Error extracting text: {}", e);
//           // Update the Db to not parse this file again.
//           continue;
//         }
//       };
//       // update the file_content in the database
//       let _ = diesel::update(document::table.filter(document::path.eq(&path)))
//         .set(document::file_content.eq(extracted_text))
//         .execute(connection)
//         .unwrap();
//     }
//   }
//   // pass the files to handle_existing_files for updating the database
// }

pub fn add_metadata_to_database(files_array: &Vec<DocumentItem>, connection: &mut SqliteConnection) {
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
  // using batch insert
  // if files_to_add.len() > 0 {
  //   let _ = diesel::insert_into(document::table)
  //     .values(files_to_add)
  //     .execute(connection)
  //     .unwrap();
  // }
  // using transaction
  if files_to_add.len() > 0 {
    connection.transaction::<_, diesel::result::Error, _>(|connection| {
        diesel::insert_into(document::table)
            .values(files_to_add)
            .execute(connection)
    }).unwrap();
  }

  // handle files to update
  if files_to_update.len() > 0 {
    println!("Handling existing file: {}", files_to_update[0].name);
    // for each file in existing_files only update the last_modified, last_opened and size fields
    for file in files_to_update {
      let _ = diesel::update(document::table.filter(document::path.eq(&file.path)))
        .set((
          document::last_modified.eq(&file.last_modified),
        ))
        .execute(connection)
        .unwrap();
    }
  }

}

// pub fn handle_existing_files(existing_files: Vec<DocumentItem>, connection: &mut SqliteConnection) {
//   println!("Handling existing file: {}", existing_files[0].name);
//   // for each file in existing_files only update the last_modified, last_opened and size fields
//   for file in existing_files {
//     let _ = diesel::update(document::table.filter(document::path.eq(&file.path)))
//       .set((
//         document::last_modified.eq(&file.last_modified),
//         document::last_opened.eq(&file.last_opened),
//         document::size.eq(&file.size),
//       ))
//       .execute(connection)
//       .unwrap();
//   }
// }