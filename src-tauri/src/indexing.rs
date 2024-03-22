// use crate::database::crud::add_files_to_database;
use crate::database::schema::{document, metadata, body, file_types};
use crate::database::models::{DocumentItem, BodyItem, FileTypes};
use crate::db_sync::sync_status;
use crate::housekeeping::get_home_directory;
use crate::ipc::send_message_to_frontend;
use crate::utils::{self, get_metadata};
use crate::text_extraction::Extractor;
use diesel::connection::Connection;
use diesel::{ExpressionMethods, QueryDsl, JoinOnDsl, RunQueryDsl, SqliteConnection};
use jwalk::{WalkDir, WalkDirGeneric};
use log::{error, info, trace, warn};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

pub fn all_allowed_filetypes(connection: &mut SqliteConnection, only_allowed: bool) -> Vec<FileTypes> {
  let filetypes = file_types::table
    .select((file_types::file_type, file_types::file_type_category, file_types::file_type_allowed, file_types::added_by_user))
    .load::<FileTypes>(connection)
    .unwrap();

  if only_allowed {
    filetypes.into_iter()
      .filter(|filetype| filetype.file_type_allowed == true)
      .collect()
  } else {
    filetypes
  }
}

fn get_all_forbidden_directories() -> Vec<String> {
    let home_dir: String = get_home_directory().unwrap();
    let mut all_forbidden_directories: Vec<String> = vec![];
    let forbidden_directories: [&str; 4] = ["node_modules", "venv", "bower_components", "pycache"];
    all_forbidden_directories.extend(forbidden_directories.iter().map(|&s| s.to_string()));
    #[cfg(target_os = "windows")]
    {
        let windows_forbidden_directories: [&str; 6] = [
            "$RECYCLE.BIN",
            "System Volume Information",
            "AppData",
            "ProgramData",
            "Windows",
            "Program Files",
        ];
        all_forbidden_directories
            .extend(windows_forbidden_directories.iter().map(|&s| s.to_string()));
    }
    #[cfg(target_os = "macos")]
    {
        let mac_forbidden_directories: [&str; 2] = [
            &format!("{}/Library", home_dir),
            &format!("{}/Applications", home_dir),
        ];
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

pub fn walk_directory(conn: &mut SqliteConnection, window: &tauri::Window, path: &str) -> usize {
    info!("Logger initialized!");
    let mut files_array: Vec<DocumentItem> = vec![];
    let all_forbidden_directories = get_all_forbidden_directories();
    let mut files_added = 0;
    let allowed_filetypes = all_allowed_filetypes(conn, true);
    let allowed_extensions: Vec<String> = allowed_filetypes
        .iter()
        .map(|filetype| filetype.file_type.to_string())
        .collect();

    let walk_dir = build_walk_dir(&path.to_string(), all_forbidden_directories);

    // for entry in WalkDir::new(path) {
    for entry in walk_dir {
        let entry = entry.unwrap();
        let path = entry.path();
        // info!("Indexing: {}", path.to_str().unwrap());

        // if the path does not exist or is not a file, continue
        if !path.exists() || !path.is_file() {
            // println!("Folder maybe?: {}", path.to_str().unwrap());
            continue;
        }

        let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let mut extension = path.extension().and_then(|s| s.to_str());

        // if extension is not in allowed filetypes, continue
        if extension.is_none() || !allowed_extensions.contains(&extension.unwrap().to_string()) {
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
        let last_modified_secs = metadata
            .modified()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let created_at = metadata
            .created()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let last_opened = metadata
            .accessed()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // If extension is None or is_folder is true, continue
        if extension.is_none() || is_folder {
            continue;
        }

        let file_item = DocumentItem {
            source_domain: "local".to_string(),
            created_at: created_at as i64,
            name: filename.to_string(),
            path: path.to_str().unwrap().to_string(),
            size: Some(filesize as f64),
            file_type: extension.unwrap().to_string(),
            last_modified: last_modified_secs as i64,
            last_opened: last_opened as i64,
            last_synced: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            is_pinned: false,
            frecency_rank: 0.0,
            frecency_last_accessed: 0,
            comment: None,
        };

        files_array.push(file_item);

        // if there are 500 items in files_array, add them to the database and clear the array
        if files_array.len() == 500 {
            add_file_metadata_to_database(&files_array, conn);
            files_added += files_array.len();
            send_message_to_frontend(
                &window,
                "files-added".to_string(),
                "files_added".to_string(),
                files_added.to_string(),
            );
            files_array.clear();
        }
    }
    // process the leftover files from the last iteration (because count may be < 500)
    if files_array.len() > 0 {
      // let cloned_files_array = files_array.clone();
      add_file_metadata_to_database(&files_array, conn);
      files_added += files_array.len();
      send_message_to_frontend(
          &window,
          "files-added".to_string(),
          "files_added_complete".to_string(),
          files_added.to_string(),
      );
      files_array.clear();
    }
    // return number of files_added
    files_added
}

pub fn add_file_metadata_to_database(
    files_array: &Vec<DocumentItem>,
    connection: &mut SqliteConnection,
) {
    let files_array_clone = files_array.clone();
    // collect all file paths from files_array
    let file_paths: Vec<_> = files_array.iter().map(|file| &file.path).collect();

    // get all existing files from the database
    let existing_files = document::table
        .select((
            document::path,
            document::last_modified,
            document::last_opened,
            document::size,
        ))
        .filter(document::path.eq_any(file_paths))
        .load::<(String, i64, i64, Option<f64>)>(connection)
        .unwrap();

    // filter files that do not exist in the database
    let files_to_add: Vec<_> = files_array
        .into_iter()
        .filter(|file| {
            !existing_files
                .iter()
                .any(|(path, _, _, _)| path == &file.path)
        })
        .collect();

    // filter files that already exist in the database
    // and whose last_modified, last_opened or size has changed
    let files_to_update: Vec<_> = files_array_clone
        .into_iter()
        .filter(|file| {
            existing_files
                .iter()
                .any(|(path, last_modified, last_opened, size)| {
                    path == &file.path
                        && (last_modified != &file.last_modified
                            || last_opened != &file.last_opened
                            || size != &file.size)
                })
        })
        .collect();

    // add the new files to the database
    if files_to_add.len() > 0 {
        // println!(">>> Adding {} new files", files_to_add.len());
        connection
            .transaction::<_, diesel::result::Error, _>(|connection| {
                diesel::insert_into(document::table)
                    .values(files_to_add)
                    .execute(connection)
            })
            .unwrap();
    }

    // handle existing files and update the database
    if files_to_update.len() > 0 {
        println!(">>> Updating {} existing files", files_to_update.len());
        // for each file in existing_files only update the last_modified, last_opened and size fields
        for file in files_to_update {
            let _ = diesel::update(document::table.filter(document::path.eq(&file.path)))
                .set((
                    document::last_modified.eq(&file.last_modified),
                    document::last_opened.eq(&file.last_opened),
                    document::size.eq(&file.size),
                ))
                .execute(connection)
                .unwrap();
        }
    }
}

pub fn parse_content_from_files(conn: &mut SqliteConnection) -> usize {
  let mut files_parsed = 0;
  let document_filetypes = ["docx", "md", "pptx", "txt", "epub"];
  // let allowed_filetypes = all_allowed_filetypes(conn, true);
  // let document_filetypes: Vec<String> = allowed_filetypes
  //   .iter()
  //   .filter(|filetype| filetype.file_type_category == "document")
  //   .map(|filetype| filetype.file_type.to_string())
  //   .collect();

  // Get metadata_id, source_id, path, file_type and last_modified
  // For all files that have the filetype in the array above
  let files_data = document::table
    .inner_join(metadata::table.on(document::id.eq(metadata::source_id)))
    .filter(document::file_type.eq_any(document_filetypes))
    .select((metadata::id, document::path, document::name, document::file_type, document::last_modified))
    .load::<(i32, String, String, String, i64)>(conn)
    .unwrap();

  let metadata_ids_to_select: Vec<i32> = files_data.iter().map(|item| item.0).collect();

  let last_parsed_values: HashMap<i32, i64> = body::table
    .filter(body::metadata_id.eq_any(metadata_ids_to_select))
    .select((body::metadata_id, body::last_parsed))
    .load::<(i32, i64)>(conn)
    .unwrap()
    .into_iter()
    .collect();
  
  let mut sync_running = sync_status(conn);

  // Then parse and chunk the content and store it in the body table
  for file_item in files_data {
    let metadata_id = file_item.0;
    let path = file_item.1;
    let name = file_item.2;
    let file_type = file_item.3;
    let last_modified = file_item.4;
    let last_parsed: Option<&i64>;
    match last_parsed_values.get(&metadata_id) {
      Some(value) => {
        // Handle the case where metadata_id exists in the HashMap
        last_parsed = Some(value);
      }
      None => {
        // Handle the case where metadata_id does not exist in the HashMap
        last_parsed = None;
      }
    }  
    // If last_parsed is None or file_item.last_modified > last_parsed, then parse the file
    if last_parsed.is_none() || last_modified > *last_parsed.unwrap_or(&0) {
      // Extract text from the file
      let text = extract_text_from_path(path.clone(), file_type.clone());
      // If there is no text, still add this file so that next time its last_parsed is compared
      // Chunk the text into 2000 character chunks
      let chunks = chunk_text(text);
      let body_items: Vec<BodyItem> = chunks
        .iter()
        .map(|chunk| {
          BodyItem {
            metadata_id: metadata_id, 
            text: chunk.to_string(),
            title: name.clone(),
            url: path.clone(),
            last_parsed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
          }
        }).collect();

      add_body_to_database(&body_items, conn);
      files_parsed += 1;
    }

    // Check if sync_running is false, if so break the loop
    if sync_running == "false" {
      break;
    }
    // Update sync_running status
    sync_running = sync_status(conn);
  }
  files_parsed
}

pub fn extract_text_from_path(path: String, file_type: String) -> String {
  let extractor: Extractor = Extractor::new();
  let extracted_text = extractor.extract_text_from_file(path, file_type);
  match extracted_text {
    Ok(text) => text,
    Err(e) => {
      eprintln!("Error extracting text: {}", e);
      String::new()
    }
  }
}

fn chunk_text(text: String) -> Vec<String> {
  // chunk the text into 2000 character chunks
  let mut chunks: Vec<String> = vec![];
  let mut chunk = String::new();
  for word in text.split_whitespace() {
    if chunk.len() + word.len() < 2000 {
      chunk.push_str(word);
      chunk.push_str(" ");
    } else {
      chunks.push(chunk);
      chunk = String::new();
    }
  }
  chunks.push(chunk);
  chunks
}

fn add_body_to_database(body_items: &Vec<BodyItem>, connection: &mut SqliteConnection) {
  // using transaction
  if body_items.len() > 0 {
    // println!("{}", body_items[0].metadata_id.to_string());
    // println!("{}", body_items[0].text);
    connection
      .transaction::<_, diesel::result::Error, _>(|connection| {
        diesel::insert_into(body::table)
          .values(body_items)
          .execute(connection)
      })
      .unwrap();
  }
}

pub fn remove_nonexistent_files(conn: &mut SqliteConnection) {
  let all_files = document::table
    .select(document::path)
    .load::<String>(conn)
    .unwrap();
  let mut nonexistent_files: Vec<String> = vec![];
  for file in all_files {
    if !std::path::Path::new(&file).exists() {
      nonexistent_files.push(file);
    }
  }
  if nonexistent_files.len() > 0 {
    let _ = diesel::delete(document::table
      .filter(document::path.eq_any(nonexistent_files)))
      .execute(conn)
      .unwrap();
  }
}