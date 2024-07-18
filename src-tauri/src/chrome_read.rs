use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::custom_types::{HistoryResult, Error};
use crate::database::models::DocumentSearchResult;

const DEFAULT_CHROME_PROFILE_ID: &str = "Default";
const DEFAULT_CHROME_PROFILE_PATH: [&str; 3] = ["Application Support", "Google", "Chrome"];
const DEFAULT_CHROME_STATE_PATH: [&str; 4] = ["Application Support", "Google", "Chrome", "Local State"];

fn user_library_directory_path() -> PathBuf {
    let home_path = dirs::home_dir().expect("Could not find home directory");
    home_path.join("Library")
}

fn get_history_db_path(profile_name: Option<&str>) -> PathBuf {
  // get the profile id from the profile name
  // let chrome_profiles = load_chrome_profiles();
  // let profile_id = Some(chrome_profiles.iter().find(|profile| profile.get("name").unwrap() == profile_name.unwrap()).unwrap().get("id").unwrap());
  // let binding = DEFAULT_CHROME_PROFILE_ID.to_string();
  // let profile = profile_id.unwrap_or(&binding);

  let profile = profile_name.unwrap_or(DEFAULT_CHROME_PROFILE_ID);
  let mut path = user_library_directory_path();
  for p in DEFAULT_CHROME_PROFILE_PATH.iter() {
      path = path.join(p);
  }
  path.join(profile).join("History")
}

fn get_local_state_path() -> PathBuf {
    let mut path = user_library_directory_path();
    for p in DEFAULT_CHROME_STATE_PATH.iter() {
        path = path.join(p);
    }
    path
}

fn _get_bookmarks_file_path(profile: Option<&str>) -> PathBuf {
    let profile = profile.unwrap_or(DEFAULT_CHROME_PROFILE_ID);
    let mut path = user_library_directory_path();
    for p in DEFAULT_CHROME_PROFILE_PATH.iter() {
        path = path.join(p);
    }
    path.join(profile).join("Bookmarks")
}

fn load_chrome_profiles() -> Vec<HashMap<String, String>> {
    let path = get_local_state_path();
    if !path.exists() {
        return vec![{
            let mut default_profile = HashMap::new();
            default_profile.insert("name".to_string(), "Default".to_string());
            default_profile.insert("id".to_string(), "Default".to_string());
            default_profile
        }];
    }

    let chrome_state = fs::read_to_string(path).expect("Could not read local state file");
    let chrome_state: serde_json::Value = serde_json::from_str(&chrome_state).expect("Invalid JSON in local state file");
    let profiles = &chrome_state["profile"]["info_cache"];
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

pub fn get_chrome_profiles() -> Vec<String> {
  let chrome_profiles = load_chrome_profiles();
  println!("{:?}", chrome_profiles);
  let profile_names = chrome_profiles.iter().map(|profile| {profile.get("name").unwrap().to_string()}).collect();

  profile_names
}

pub fn search_chrome(profile: String, user_query: String, limit: i64, page: i64) -> Result<Vec<DocumentSearchResult>, Error> {
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
      source_domain: "Chrome".to_string(),
      created_at: 0,
      name: title.clone(),
      path: url.clone(),
      size: None,
      file_type: "chrome-webpage".to_string(),
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
