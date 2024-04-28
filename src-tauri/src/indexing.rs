use crate::custom_types::Error;
use crate::database::schema::{document, metadata, body, metadata_fts, body_fts, ignore_list, allow_list, file_types};
use crate::database::models::{BodyItem, DocumentItem, FileTypes, IgnoreList, AllowList};
use crate::db_sync::sync_status;
use crate::housekeeping::get_home_directory;
use crate::ipc::send_message_to_frontend;
use crate::utils::{self, get_metadata};
use crate::text_extraction::Extractor;
use diesel::connection::Connection;
use diesel::{ExpressionMethods, QueryDsl, JoinOnDsl, RunQueryDsl, SqliteConnection};
use jwalk::{WalkDir, WalkDirGeneric};
// use log::{info, error};
use std::path::PathBuf;
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
      let home_dir: String = get_home_directory().unwrap();
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
                    // info!("skip path {}", curr_path);
                    dir_entry.read_children_path = None;
                }
            }
        });
    })
}

pub fn create_document_item(file_path: PathBuf, allowed_extensions: &Vec<String>) -> Result<DocumentItem, Error> {
  // if the path does not exist or is not a file, continue
  if !file_path.exists() || !file_path.is_file() {
      // println!("Folder maybe?: {}", path.to_str().unwrap());
      return Err(Error::new("Path does not exist or is not a file"));
  }

  let filename = file_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
  let mut extension = file_path.extension().and_then(|s| s.to_str());

  // if extension is not in allowed filetypes, continue
  if extension.is_none() || !allowed_extensions.contains(&extension.unwrap().to_string()) {
      // println!("ignoring file");
      return Err(Error::new("Extension is not in allowed filetypes"));
  }
  // if filename starts with a dot or ~$, continue
  if filename.starts_with(".") || filename.starts_with("~$") {
      // println!("ignoring file");
      return Err(Error::new("Filename starts with a dot or ~$"));
  }

  let metadata = get_metadata(&file_path).unwrap();
  // if metadata is a symlink or shortcut file, continue
  if metadata.file_type().is_symlink() {
      // println!("ignoring shortcut");
      return Err(Error::new("File is a symlink"));
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
    return Err(Error::new("Extension is None or is_folder is true"));
  }

  let file_item = DocumentItem {
    source_domain: "local".to_string(),
    created_at: created_at as i64,
    name: filename.to_string(),
    path: file_path.to_str().unwrap().to_string(),
    size: Some(filesize as f64),
    file_type: extension.unwrap().to_string(),
    last_modified: last_modified_secs as i64,
    last_opened: last_opened as i64,
    last_synced: SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs() as i64,
    last_parsed: 0,
    is_pinned: false,
    frecency_rank: 0.0,
    frecency_last_accessed: 0,
    comment: None,
  };

  Ok(file_item)
}

pub fn walk_directory(conn: &mut SqliteConnection, window: &tauri::WebviewWindow, file_paths: Vec<String>) -> usize {
    let mut files_array: Vec<DocumentItem> = vec![];
    let all_forbidden_directories = get_all_forbidden_directories();
    let mut files_added = 0;
    let allowed_filetypes = all_allowed_filetypes(conn, true);
    let allowed_extensions: Vec<String> = allowed_filetypes
        .iter()
        .map(|filetype| filetype.file_type.to_string())
        .collect();
    let ignored_items = get_all_ignored_paths(conn);
    let ignored_files: Vec<IgnoreList> = ignored_items.iter().filter(|item| !item.is_folder).cloned().collect();
    let ignored_folders: Vec<IgnoreList> = ignored_items.iter().filter(|item| item.is_folder).cloned().collect();
    let allowed_items = get_all_allowed_paths(conn);
    let allowed_files: Vec<AllowList> = allowed_items.iter().filter(|item| !item.is_folder).cloned().collect();
    let allowed_folders: Vec<AllowList> = allowed_items.iter().filter(|item| item.is_folder).cloned().collect();

    for path in file_paths {
      println!("Indexing file path: {}", path);
      let walk_dir = build_walk_dir(&path, all_forbidden_directories.clone());
      
      for entry in walk_dir {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        // info!("Indexing: {}", path.to_str().unwrap());

        let file_item = create_document_item(entry_path, &allowed_extensions);
        let file_item = match file_item {
          Ok(file_item) => file_item,
          Err(_e) => {
            // error!("Error creating document item: {}", e);
            continue;
          }
        };

        // if file is in the ignored_files list and ignore_indexing is true
        if ignored_files.iter().any(|item| item.path == file_item.path && item.ignore_indexing) {
          // if file is not in allowed_files or file path does not start with a path in allowed_folders, continue
          if !allowed_files.iter().any(|item| item.path == file_item.path) && !allowed_folders.iter().any(|item| file_item.path.starts_with(&item.path)) {
            continue;
          }
        }
        // if file path starts with a path in the ignored_folders
        if ignored_folders.iter().any(|item| file_item.path.starts_with(&item.path) && item.ignore_indexing) {
          // if file is not in allowed_files or file path does not start with a path in allowed_folders, continue
          if !allowed_files.iter().any(|item| item.path == file_item.path) && !allowed_folders.iter().any(|item| file_item.path.starts_with(&item.path)) {
            continue;
          }
        }

        // if all clear, add file_item to files_array
        files_array.push(file_item);

        // if there are 500 items in files_array, add them to the database and clear the array
        if files_array.len() == 500 {
          add_file_metadata_to_database(&files_array, conn);
          files_added += files_array.len();
          // This message gives incremental updates to the frontend
          // And is necessary for setting dbReady = true in the frontend
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
        // This message sets onboardingDone = true in the frontend
        send_message_to_frontend(
            &window,
            "files-added".to_string(),
            "files_added_complete".to_string(),
            files_added.to_string(),
        );
        files_array.clear();
      }
    }

    // remove files from the database that do not exist in the filesystem
    remove_nonexistent_and_ignored_files(conn);
    // add folders to the database
    add_folders_to_db(conn);
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

pub async fn parse_content_from_files(conn: &mut SqliteConnection, app: tauri::AppHandle) -> usize {
  let mut files_parsed = 0;

  let document_filetypes = ["docx", "md", "pptx", "txt", "epub"];
  // let document_filetypes: [&str; 1] = ["md"];
  println!("Document filetypes: {:?}", document_filetypes);
  
  // let allowed_filetypes = all_allowed_filetypes(conn, true);
  // let document_filetypes: Vec<String> = allowed_filetypes
  //   .iter()
  //   .filter(|filetype| filetype.file_type_category == "document")
  //   .map(|filetype| filetype.file_type.to_string())
  //   .collect();


  let ignored_items = get_all_ignored_paths(conn);
  let ignored_files: Vec<IgnoreList> = ignored_items.iter().filter(|item| !item.is_folder).cloned().collect();
  let ignored_folders: Vec<IgnoreList> = ignored_items.iter().filter(|item| item.is_folder).cloned().collect();
  
  let allowed_items = get_all_allowed_paths(conn);
  let allowed_files: Vec<AllowList> = allowed_items.iter().filter(|item| !item.is_folder).cloned().collect();
  let allowed_folders: Vec<AllowList> = allowed_items.iter().filter(|item| item.is_folder).cloned().collect();
  
  // Get metadata_id, source_id, path, file_type and last_modified
  // For all files that have the filetype in the array above
  let not_pdf_files_data = document::table
    .inner_join(metadata::table.on(document::id.eq(metadata::source_id)))
    .filter(document::file_type.eq_any(document_filetypes))
    .select((metadata::id, document::path, document::name, document::file_type, document::last_modified))
    .order_by(document::size.asc())
    .load::<(i32, String, String, String, i64)>(conn)
    .unwrap();
  // Get the same for all PDF files
  let pdf_files_data = document::table
    .inner_join(metadata::table.on(document::id.eq(metadata::source_id)))
    .filter(document::file_type.eq_any(["pdf"]))
    .select((metadata::id, document::path, document::name, document::file_type, document::last_modified))
    .load::<(i32, String, String, String, i64)>(conn)
    .unwrap();
  
  println!("PDF files: {}", pdf_files_data.len());
  println!("Not PDF files: {}", not_pdf_files_data.len());
  
  // Append the pdf_files_data to not_pdf_files_data
  let all_files_data = not_pdf_files_data.clone();
  let all_files_data: Vec<(i32, String, String, String, i64)> = all_files_data.into_iter().chain(pdf_files_data.into_iter()).collect();
  
  let all_files_data: Vec<(i32, String, String, String, i64)> = all_files_data.into_iter().filter(|item| {
    let path = item.1.clone();
    // Check if the file is in the allowed_files list
    if allowed_files.iter().any(|allowed_file| path.contains(&allowed_file.path)) {
      return true;
    }
    // Check if the file path starts with a path in the allowed_folders list
    if allowed_folders.iter().any(|allowed_folder| path.starts_with(&allowed_folder.path)) {
      return true;
    }
    // Check if the file is in the ignored_files list
    if ignored_files.iter().any(|ignored_file| path.contains(&ignored_file.path)) {
      return false;
    }
    // Check if the file path starts with a path in the ignored_folders list
    if ignored_folders.iter().any(|ignored_folder| path.starts_with(&ignored_folder.path)) {
      return false;
    }
    true
  }).collect();
  
  let metadata_ids_to_select: Vec<i32> = all_files_data.iter().map(|item| item.0).collect();
  let all_last_parsed_values: HashMap<i32, i64> = body::table
    .filter(body::metadata_id.eq_any(metadata_ids_to_select))
    .select((body::metadata_id, body::last_parsed))
    .load::<(i32, i64)>(conn)
    .unwrap()
    .into_iter()
    .collect();

  // Get sync_running status
  let mut sync_running = sync_status(&app).0;

  // Iterate over all_files_data and extract text from each file
  for file_item in all_files_data {
    let metadata_id = file_item.0;
    let path = file_item.1;
    let name = file_item.2;
    let file_type = file_item.3;
    let last_modified = file_item.4;
    let last_parsed: Option<&i64>;
    match all_last_parsed_values.get(&metadata_id) {
      Some(value) => {
        // Handle the case where metadata_id exists in the HashMap
        last_parsed = Some(value);
      }
      None => {
        // Handle the case where metadata_id does not exist in the HashMap
        last_parsed = None;
      }
    }
    // 1. BEFORE EXTRACTING TEXT: Break the loop if sync_running is false
    if sync_running == "false" {
      break;
    }
    // If last_parsed is None or file_item.last_modified > last_parsed, then parse the file
    if last_parsed.is_none() || last_modified > *last_parsed.unwrap_or(&0) {
      // Extract text from the file
      // info!("Extracting text from: {}", path.clone());
      let text = extract_text_from_path(path.clone(), file_type.clone(), &app).await;
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
      // update last_parsed in document table for this file
      let _ = diesel::update(document::table.filter(document::id.eq(metadata_id)))
        .set(document::last_parsed.eq(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64))
        .execute(conn)
        .unwrap();
      files_parsed += 1;
    }
    // 2. AFTER ADDING TO DB: Break the loop if sync_running is false
    if sync_running == "false" {
      break;
    }
    // Update sync_running status
    sync_running = sync_status(&app).0;
  }

  files_parsed
}

pub async fn extract_text_from_path(path: String, file_type: String, app: &tauri::AppHandle) -> String {
  let extractor: Extractor = Extractor::new();
  let extracted_text = extractor.extract_text_from_file(path, file_type, app).await;
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

pub fn remove_nonexistent_and_ignored_files(conn: &mut SqliteConnection) {
  let all_file_paths = document::table
    .select(document::path)
    .load::<String>(conn)
    .unwrap();

  // add all files whose path is in the ignore_list
  let ignored_items = get_all_ignored_paths(conn);
  // divide ignored_items into ignored_files and ignored_folders based on is_folder
  let ignored_files: Vec<IgnoreList> = ignored_items.iter().filter(|item| !item.is_folder).cloned().collect();
  let ignored_folders: Vec<IgnoreList> = ignored_items.iter().filter(|item| item.is_folder).cloned().collect();

  let allowed_items = get_all_allowed_paths(conn);
  let allowed_files: Vec<AllowList> = allowed_items.iter().filter(|item| !item.is_folder).cloned().collect();
  let allowed_folders: Vec<AllowList> = allowed_items.iter().filter(|item| item.is_folder).cloned().collect();

  println!("All files: {}", &all_file_paths.len());

  let mut files_to_remove: Vec<String> = vec![];
  for path in all_file_paths {
    // if path does not exist, add it to files_to_remove
    if !std::path::Path::new(&path).exists() {
      files_to_remove.push(path.clone());
    }
    // if path is in the allowed_files list, continue
    if allowed_files.iter().any(|item| item.path == *path) {
      continue;
    }
    // if path starts with a folder in the allowed_folders list, continue
    if allowed_folders.iter().any(|item| path.starts_with(&item.path)) {
      continue;
    }
    // if path is in the ignored_files list, continue
    if ignored_files.iter().any(|item| item.path == *path) {
      files_to_remove.push(path.clone());
    }
    // if path starts with a folder in the ignored_folders list, continue
    if ignored_folders.iter().any(|item| path.starts_with(&item.path)) {
      files_to_remove.push(path.clone());
    }
  }

  println!("Files to remove: {}", files_to_remove.len());

  if files_to_remove.len() > 0 {
    println!("Removing {} files", files_to_remove.len());
    // create transactions of 100 files each
    let mut chunked_files_to_remove: Vec<Vec<String>> = vec![];
    let mut chunk: Vec<String> = vec![];
    for file in files_to_remove {
      if chunk.len() == 100 {
        chunked_files_to_remove.push(chunk.clone());
        chunk.clear();
      }
      chunk.push(file);
    }
    chunked_files_to_remove.push(chunk.clone());
    // remove files from the database
    for chunks_of_files_to_remove in chunked_files_to_remove {
      println!("Removing {} files from chunk", chunks_of_files_to_remove.len());
      remove_vector_of_file_paths_from_db(chunks_of_files_to_remove, conn);
    }
  }
}

fn remove_vector_of_file_paths_from_db(file_paths: Vec<String>, conn: &mut SqliteConnection) {
  let file_paths_clone = file_paths.clone();
  // get metadata_id for all file_paths
  let metadata_ids = document::table
    .inner_join(metadata::table.on(document::id.eq(metadata::source_id)))
    .filter(document::path.eq_any(file_paths_clone))
    .select(metadata::id)
    .load::<i32>(conn)
    .unwrap();

  // first delete from body_fts, then body, then metadata_fts, then metadata, then document
  // delete from body_fts
  conn.transaction::<_, diesel::result::Error, _>(|connection| {
    diesel::delete(body_fts::table.filter(body_fts::metadata_id.eq_any(metadata_ids.clone())))
      .execute(connection)
  }).unwrap();
  // delete from body
  conn.transaction::<_, diesel::result::Error, _>(|connection| {
    diesel::delete(body::table.filter(body::metadata_id.eq_any(metadata_ids.clone())))
      .execute(connection)
  }).unwrap();
  // delete from metadata_fts
  conn.transaction::<_, diesel::result::Error, _>(|connection| {
    diesel::delete(metadata_fts::table.filter(metadata_fts::id.eq_any(metadata_ids.clone())))
      .execute(connection)
  }).unwrap();
  // delete from metadata
  conn.transaction::<_, diesel::result::Error, _>(|connection| {
    diesel::delete(metadata::table.filter(metadata::id.eq_any(metadata_ids.clone())))
      .execute(connection)
  }).unwrap();
  // delete from document
  conn.transaction::<_, diesel::result::Error, _>(|connection| {
    diesel::delete(document::table.filter(document::path.eq_any(file_paths)))
      .execute(connection)
  }).unwrap();
}

pub fn add_folders_to_db(conn: &mut SqliteConnection) {
  // Get all file paths from the document table (excluding folders)
  let all_files = document::table
    .select(document::path)
    .filter(document::file_type.ne("folder"))
    .load::<String>(conn)
    .unwrap();

  // Get parent folders for all the files
  let all_folders: Vec<String> = all_files
    .iter()
    .map(|file| std::path::Path::new(file).parent().unwrap().to_str().unwrap().to_string())
    .collect();
  
  println!("All folders (= Num files): {}", all_folders.len());
  // Get all existing folders from the database
  let existing_folders = document::table
    .select(document::path)
    .filter(document::file_type.eq("folder"))
    .load::<String>(conn)
    .unwrap();

  // Iterate over all_folders and add only unique folders
  let mut unique_folders: Vec<String> = vec![];
  all_folders.iter().for_each(|folder| {
    if !unique_folders.contains(&folder) && !existing_folders.contains(&folder){
      unique_folders.push(folder.to_string());
    }
  });
  println!("Unique folders: {}", unique_folders.len());

  if unique_folders.len() == 0 {
    return;
  }
  // Get metadata for each folder and add it to the document table
  let folder_items: Vec<DocumentItem> = unique_folders
    .iter()
    .map(|folder| {
      let folder_metadata = get_metadata(&std::path::Path::new(folder)).unwrap();
      DocumentItem {
        source_domain: "local".to_string(),
        created_at: folder_metadata.created().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        name: std::path::Path::new(folder).file_name().unwrap().to_str().unwrap().to_string(),
        path: folder.to_string(),
        size: None,
        file_type: "folder".to_string(),
        last_modified: folder_metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        last_opened: folder_metadata.accessed().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        last_synced: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        last_parsed: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        is_pinned: false,
        frecency_rank: 0.0,
        frecency_last_accessed: 0,
        comment: None,
      }
    })
    .collect();
  let _ = diesel::insert_into(document::table)
    .values(folder_items)
    .execute(conn)
    .unwrap();
}

pub fn add_path_to_ignore_list(path: String, is_folder: bool, ignore_indexing: bool, ignore_content: bool, conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
  // remove path from allow_list if it exists
  let _ = diesel::delete(allow_list::table.filter(allow_list::path.eq(path.clone()))).execute(conn);
  // add path to ignore_list
  diesel::insert_into(ignore_list::table)
    .values(IgnoreList {
      path,
      is_folder,
      ignore_indexing,
      ignore_content
    })
    .execute(conn)
}

pub fn get_all_ignored_paths(conn: &mut SqliteConnection) -> Vec<IgnoreList> {
  // get all columns from ignore_list except id
  ignore_list::table
    .select((
      ignore_list::path,
      ignore_list::is_folder,
      ignore_list::ignore_indexing,
      ignore_list::ignore_content
    ))
    .load::<IgnoreList>(conn)
    .unwrap()
}

pub fn add_path_to_allow_list(path: String, is_folder: bool, conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
  // remove path from ignore_list if it exists
  let _ = diesel::delete(ignore_list::table.filter(ignore_list::path.eq(path.clone()))).execute(conn);
  // add path to allow_list
  diesel::insert_into(allow_list::table)
    .values(AllowList {
      path,
      is_folder
    })
    .execute(conn)
}

pub fn get_all_allowed_paths(conn: &mut SqliteConnection) -> Vec<AllowList> {
  // get all columns from allow_list except id
  allow_list::table
    .select((
      allow_list::path,
      allow_list::is_folder,
    ))
    .load::<AllowList>(conn)
    .unwrap()
}