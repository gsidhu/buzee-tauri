use rusqlite::{Connection, Result, OpenFlags};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use crate::custom_types::{Error, HistoryResult};
use crate::database::models::DocumentSearchResult;

fn user_data_directory_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    home_dir.join("Library").join("Application Support").join("Firefox").join("Profiles")
}

fn get_profile_name(user_directory_path: &Path) -> Option<String> {
    let profiles = fs::read_dir(user_directory_path).expect("Could not read user directory");

    let mut release_profile = None;
    let mut nightly_profile = None;

    for profile in profiles {
        if let Ok(profile) = profile {
            let profile_name = profile.file_name().into_string().expect("Invalid profile name");
            if profile_name.ends_with(".default-release") {
                release_profile = Some(profile_name);
            } else if profile_name.ends_with(".default-nightly") {
                nightly_profile = Some(profile_name);
            }
        }
    }

    release_profile.or(nightly_profile)
}

fn get_history_db_path() -> Option<PathBuf> {
    let user_directory_path = user_data_directory_path();
    get_profile_name(&user_directory_path).map(|profile_name| {
        user_directory_path.join(profile_name).join("places.sqlite")
    })
}

fn _get_bookmarks_directory_path() -> Option<PathBuf> {
    let user_directory_path = user_data_directory_path();
    get_profile_name(&user_directory_path).map(|profile_name| {
        user_directory_path.join(profile_name).join("bookmarkbackups")
    })
}

fn where_clauses(terms: &[&str]) -> String {
    terms.iter().map(|&term| format!("(moz_places.title LIKE '%{}%' OR moz_places.url LIKE '%{}%')", term, term)).collect::<Vec<_>>().join(" AND ")
}

fn get_history_query(query: Option<&str>, limit: i64, offset: i64) -> String {
    let terms: Vec<&str> = query.map_or(vec![], |q| q.trim().split_whitespace().collect());
    let where_clause = if terms.is_empty() {
        "".to_string()
    } else {
        format!("WHERE {}", where_clauses(&terms))
    };
    format!(
        "
        SELECT
            id, url, title,
            datetime(last_visit_date/1000000, 'unixepoch') as last_visited
        FROM moz_places
        {}
        ORDER BY last_visit_date DESC LIMIT {} OFFSET {};
        ",
        where_clause, limit, offset
    )
}

fn open_connection_with_retries(db_path: &Path, retries: usize, delay: Duration) -> Result<Connection> {
  // print the db_path
  println!("db_path: {:?}", db_path);
  for _ in 0..retries {
      match Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY) {
          Ok(conn) => return Ok(conn),
          Err(rusqlite::Error::SqliteFailure(_, Some(ref msg))) if msg.contains("database is locked") => {
              thread::sleep(delay);
          }
          Err(err) => return Err(err),
      }
  }
  Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
}

fn use_history_search(query: Option<&str>, limit: i64, offset: i64) -> HistoryResult {
    let db_path = match get_history_db_path() {
        Some(path) => path,
        None => {
            return HistoryResult {
                data: vec![],
                is_loading: false,
                error_view: Some("NotInstalledError".to_string()),
            }
        }
    };

    if !db_path.exists() {
        return HistoryResult {
            data: vec![],
            is_loading: false,
            error_view: Some("NotInstalledError".to_string()),
        }
    }

    // create a backup of the database
    create_copy_of_firefox_history_database().expect("Could not create a backup of the Firefox history database");

    // connect to the backup database
    let db_path = db_path.with_file_name("history.sqlite");
    let conn = match open_connection_with_retries(&db_path, 5, Duration::from_millis(100)) {
        Ok(conn) => conn,
        Err(err) => {
            return HistoryResult {
                data: vec![],
                is_loading: false,
                error_view: Some(err.to_string()),
            }
        }
    };

    let in_query = get_history_query(query, limit, offset);
    let mut stmt = match conn.prepare(&in_query) {
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

fn create_copy_of_firefox_history_database() -> Result<(), Error> {
  let db_path = match get_history_db_path() {
    Some(path) => path,
    None => {
      return Err(Error::new("Could not find Firefox history database"));
    }
  };

  // create a backup in the same directory
  let backup_path = db_path.with_file_name("history.sqlite");
  fs::copy(&db_path, &backup_path)?;

  Ok(())
}

pub fn search_firefox(user_query: String, limit: i64, page: i64) -> Result<Vec<DocumentSearchResult>, Error> {
  let history_result = use_history_search(Some(user_query.as_str()), limit, limit*page);
  println!("history_result: {:?}", history_result);
  let search_results: Vec<DocumentSearchResult> = history_result.data.iter().map(|(_id, url, title, last_visited)| {
    // convert last_visited to UNIX timestamp
    let last_opened = chrono::NaiveDateTime::parse_from_str(last_visited, "%Y-%m-%d %H:%M:%S").unwrap();
    // convert NaiveDateTime to UNIX timestamp
    let last_opened = last_opened.and_utc().timestamp();
    DocumentSearchResult {
      id: 0,
      source_domain: "Firefox".to_string(),
      created_at: 0,
      name: title.clone(),
      path: url.clone(),
      size: None,
      file_type: "firefox-webpage".to_string(),
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
