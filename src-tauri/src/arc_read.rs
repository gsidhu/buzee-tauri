use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::custom_types::{HistoryResult, Error};
use crate::database::models::DocumentSearchResult;

const DEFAULT_ARC_PROFILE_ID: &str = "Default";

#[cfg(target_os = "macos")]
const DEFAULT_ARC_PROFILE_PATH: [&str; 3] = ["Application Support", "Arc", "User Data"];
#[cfg(target_os = "macos")]
const DEFAULT_ARC_STATE_PATH: [&str; 4] = ["Application Support", "Arc", "User Data", "Local State"];

#[cfg(target_os = "windows")]
const DEFAULT_ARC_PROFILE_PATH: [&str; 3] = ["Roaming", "Arc", "User Data"];
#[cfg(target_os = "windows")]
const DEFAULT_ARC_STATE_PATH: [&str; 4] = ["Roaming", "Arc", "User Data", "Local State"];

fn user_library_directory_path() -> PathBuf {
    let home_path = dirs::home_dir().expect("Could not find home directory");
    home_path.join("Library")
}

fn get_history_db_path(profile_name: Option<&str>) -> PathBuf {
  // get the profile id from the profile name
  // let arc_profiles = load_arc_profiles();
  // let profile_id = Some(arc_profiles.iter().find(|profile| profile.get("name").unwrap() == profile_name.unwrap()).unwrap().get("id").unwrap());
  // let binding = DEFAULT_ARC_PROFILE_ID.to_string();
  // let profile = profile_id.unwrap_or(&binding);

  let profile = profile_name.unwrap_or(DEFAULT_ARC_PROFILE_ID);
  let mut path = user_library_directory_path();
  for p in DEFAULT_ARC_PROFILE_PATH.iter() {
      path = path.join(p);
  }
  path.join(profile).join("History")
}

fn get_local_state_path() -> PathBuf {
    let mut path = user_library_directory_path();
    for p in DEFAULT_ARC_STATE_PATH.iter() {
        path = path.join(p);
    }
    path
}

fn _get_bookmarks_file_path(profile: Option<&str>) -> PathBuf {
    let profile = profile.unwrap_or(DEFAULT_ARC_PROFILE_ID);
    let mut path = user_library_directory_path();
    for p in DEFAULT_ARC_PROFILE_PATH.iter() {
        path = path.join(p);
    }
    path.join(profile).join("Bookmarks")
}

fn load_arc_profiles() -> Vec<HashMap<String, String>> {
    let path = get_local_state_path();
    if !path.exists() {
        return vec![{
            let mut default_profile = HashMap::new();
            default_profile.insert("name".to_string(), "Default".to_string());
            default_profile.insert("id".to_string(), "Default".to_string());
            default_profile
        }];
    }

    let arc_state = fs::read_to_string(path).expect("Could not read local state file");
    let arc_state: serde_json::Value = serde_json::from_str(&arc_state).expect("Invalid JSON in local state file");
    let profiles = &arc_state["profile"]["info_cache"];
    let mut result = Vec::new();

    if let serde_json::Value::Object(profiles) = profiles {
        for (key, val) in profiles {
            if let serde_json::Value::Object(val) = val {
                if let Some(name) = val.get("name").and_then(|n| n.as_str()) {
                    let mut profile = HashMap::new();
                    profile.insert("name".to_string(), name.to_string());
                    profile.insert("id".to_string(), key.clone());
                    result.push(profile);
                }
            }
        }
    }

    result
}

fn where_clauses(table_title: &str, terms: &[&str]) -> String {
    terms.iter().map(|&term| format!("({}.title LIKE '%{}%' OR {}.url LIKE '%{}%')", table_title, term, table_title, term)).collect::<Vec<_>>().join(" AND ")
}

fn get_history_query(table: &str, terms: &[&str], limit: i64, offset: i64) -> String {
    let where_clauses_string = where_clauses(table, terms);
    format!(
        "SELECT id, url, title, datetime(last_visit_time / 1000000 + (strftime('%s', '1601-01-01')), 'unixepoch', 'localtime') as last_visited \
        FROM {} {} ORDER BY last_visit_time DESC LIMIT {} OFFSET {};",
        table,
        if where_clauses_string.is_empty() { "".to_string() } else { format!("WHERE {}", where_clauses_string) },
        limit,
        offset
    )
}

fn search_history(profile: &str, query: Option<&str>, limit: i64, offset: i64) -> HistoryResult {
    let terms: Vec<&str> = query.unwrap_or("").trim().split_whitespace().collect();
    let query = get_history_query("urls", &terms, limit, offset);
    let db_path = get_history_db_path(Some(profile));

    println!("db_path: {:?}", db_path);

    if !db_path.exists() {
        return HistoryResult {
            data: vec![],
            is_loading: false,
            error_view: Some("NotInstalledError".to_string()),
        }
    }

    // create a backup of the database
    create_copy_of_arc_history_database(profile).expect("Could not create a backup of the Arc history database");

    // connect to the backup database
    let db_path = db_path.with_file_name("HistoryBackup");

    let conn = match Connection::open(&db_path) {
        Ok(conn) => conn,
        Err(err) => {
            return HistoryResult {
                data: vec![],
                is_loading: false,
                error_view: Some(err.to_string()),
            }
        }
    };

    let mut stmt = match conn.prepare(&query) {
        Ok(stmt) => stmt,
        Err(err) => {
            return HistoryResult {
                data: vec![],
                is_loading: false,
                error_view: Some(err.to_string()),
            }
        }
    };

    let history_iter = match stmt.query_map([], |row| {
      Ok((
          row.get(0)?,
          row.get(1)?,
          row.get(2)?,
          row.get(3)?,
      ))
    }) {
        Ok(iter) => iter,
        Err(err) => {
            return HistoryResult {
                data: vec![],
                is_loading: false,
                error_view: Some(err.to_string()),
            }
        }
    };

    let data: Vec<(i64, String, String, String)> = history_iter.filter_map(Result::ok).collect();

    HistoryResult {
        data,
        is_loading: false,
        error_view: None,
    }
}

pub fn get_arc_profiles() -> Vec<String> {
  let arc_profiles = load_arc_profiles();
  println!("{:?}", arc_profiles);
  let profile_names = arc_profiles.iter().map(|profile| {profile.get("name").unwrap().to_string()}).collect();

  profile_names
}

fn create_copy_of_arc_history_database(profile: &str) -> Result<(), Error> {
  let db_path = get_history_db_path(Some(profile));
  // create a backup in the same directory
  let backup_path = db_path.with_file_name("HistoryBackup");
  fs::copy(&db_path, &backup_path)?;

  Ok(())
}

pub fn search_arc(profile: String, user_query: String, limit: i64, page: i64) -> Result<Vec<DocumentSearchResult>, Error> {
  let history_result = search_history(profile.as_str(), Some(user_query.as_str()), limit, limit*page);
  println!("{:?}", history_result);
  let search_results: Vec<DocumentSearchResult> = history_result.data.iter().map(|(_id, url, title, last_visited)| {
    // convert last_visited to UNIX timestamp
    let last_opened = chrono::NaiveDateTime::parse_from_str(last_visited, "%Y-%m-%d %H:%M:%S").unwrap();
    // convert NaiveDateTime to UNIX timestamp
    let mut last_opened = last_opened.and_utc().timestamp();
    if last_opened == -11644453800 {
      last_opened = 0;
    }
    DocumentSearchResult {
      id: 0,
      source_domain: "Arc".to_string(),
      created_at: 0,
      name: title.clone(),
      path: url.clone(),
      size: None,
      file_type: "arc-webpage".to_string(),
      last_modified: 0,
      last_opened: last_opened,
      last_parsed: 0,
      last_synced: 0,
      frecency_last_accessed: 0,
      frecency_rank: 0.0,
      is_pinned: false,
      comment: None,
    }
  }).collect();

  Ok(search_results)
}
