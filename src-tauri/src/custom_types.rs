use serde::{Serialize, Deserialize};

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error)
}

impl Error {
  pub fn new(message: &str) -> Self {
    Self::Io(std::io::Error::new(std::io::ErrorKind::Other, message))
  }
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

// This struct is for INSERTING documents into the Tantivy Index
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TantivyDocumentItem {
    pub source_id: i64,
    pub source_table: String,
    pub source_domain: String,
    pub name: String,
    pub url: String,
    pub body: String,
    pub file_type: String,
    pub last_modified: i64,
    pub comment: String,
}

// Struct for TantivyDocumentSearchResult
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TantivyDocumentSearchResult {
  pub id: i64,
  pub last_modified: i64,
}

// Struct for TantivyBrowserHistorySearchResult
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TantivyBrowserHistorySearchResult {
  pub id: i64,
  pub source_table: String,
  pub source_domain: String,
  pub is_pinned: Option<bool>,
  pub comment: Option<String>,
  pub title: Option<String>,
  pub body: Option<String>,
  pub url: Option<String>,
  pub last_visited: Option<i64>,
  pub frecency_rank: Option<f64>,
  pub frecency_last_accessed: Option<i64>,
}

// Struct for TantivyBookmarkSearchResult
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TantivyBookmarkSearchResult {
  pub id: i64,
  pub source_table: String,
  pub source_domain: String,
  pub is_pinned: Option<bool>,
  pub comment: Option<String>,
  pub title: Option<String>,
  pub body: Option<String>,
  pub url: Option<String>,
  pub saved_at: Option<i64>,
  pub last_opened: Option<i64>,
  pub word_count: Option<i64>,
  pub is_favorite: Option<bool>,
  pub is_archived: Option<bool>,
  pub is_read: Option<bool>,
  pub tags: Option<String>,
  pub frecency_rank: Option<f64>,
  pub frecency_last_accessed: Option<i64>,
}

// Struct for TantivyEmailSearchResult
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TantivyEmailSearchResult {
  pub id: i64,
  pub source_table: String,
  pub source_domain: String,
  pub is_pinned: Option<bool>,
  pub comment: Option<String>,
  pub subject: Option<String>,
  pub body: Option<String>,
  pub url: Option<String>,
  pub sender: Option<String>,
  pub recipient: Option<String>,
  pub cc: Option<String>,
  pub bcc: Option<String>,
  pub attachments: Option<String>,
  pub tags: Option<String>,
  pub frecency_rank: Option<f64>,
  pub frecency_last_accessed: Option<i64>,
}

// DateLimit struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateLimit {
  pub start: String,
  pub end: String,
  pub text: String
}

// Query Segments struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuerySegments {
  #[serde(rename = "quotedSegments")]
  pub quoted_segments: Vec<String>,
  #[serde(rename = "greedySegments")]
  pub greedy_segments: Vec<String>,
  #[serde(rename = "notSegments")]
  pub not_segments: Vec<String>,
}

// Payload for IPC events
#[derive(Clone, Serialize)]
pub struct Payload {
  pub message: String,
  pub data: String
}

// Define a struct for passing DB Stats
#[derive(Debug, Serialize, Deserialize)]
pub struct DBStat {
  pub file_type: String,
  pub count: i64
}

// // Struct for AppHandle
// pub(crate) struct AppHandleState {
//   pub stored_app_handle: tauri::AppHandle
// }

// impl AppHandleState {
//   // using new because Default doesn't let you pass arguments
//   pub fn new(app_handle: tauri::AppHandle) -> Self {
//     Self {
//       stored_app_handle: app_handle
//     }
//   }
// }

// Struct for Database Connection
// pub(crate) struct DBConnectionState {
//   pub stored_db_conn: PooledConnection<ConnectionManager<SqliteConnection>>
// }

// impl DBConnectionState {
//   // using new because Default doesn't let you pass arguments
//   pub fn new(mut conn: PooledConnection<ConnectionManager<SqliteConnection>>) -> Self {
//     Self {
//       stored_db_conn: conn
//     }
//   }
// }


use tantivy::IndexReader;
// Struct for Tantivy Reader
pub(crate) struct TantivyReaderState {
  pub reader: IndexReader
}

impl TantivyReaderState {
  pub fn new(given_reader: IndexReader) -> Self {
    Self {
      reader: given_reader
    }
  }
}

use diesel::r2d2::{Pool, ConnectionManager};
use diesel::SqliteConnection;

// Struct for Database Connection Pool
pub(crate) struct DBConnPoolState {
  pub conn_pool: Pool<ConnectionManager<SqliteConnection>>
}

impl DBConnPoolState {
  pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
    Self {
      conn_pool: pool
    }
  }
}

// Struct for Sync Running State
pub(crate) struct SyncRunningState {
  pub sync_running: bool,
  pub last_sync_time: i64
}

impl Default for SyncRunningState {
  fn default() -> Self {
    Self {
      sync_running: false,
      last_sync_time: 0
    }
  }
}

// Struct for Global Shortcut String
#[derive(Serialize, Clone)]
pub(crate) struct GlobalShortcutState {
  pub shortcut_string: String,
  pub shortcut_enabled: bool
}

impl Default for GlobalShortcutState {
    fn default() -> Self {
        Self {
          shortcut_string: "Alt+Space".to_string(),
          shortcut_enabled: false
        }
    }
}

// Struct for User Preference
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct UserPreferencesState {
  pub first_launch_done: bool,
  pub onboarding_done: bool,
  pub show_search_suggestions: bool,
  pub launch_at_startup: bool,
  pub show_in_dock: bool,
  pub global_shortcut_enabled: bool,
  pub global_shortcut: String,
  pub automatic_background_sync: bool,
  pub detailed_scan: bool,
  pub roadmap_survey_answered: bool,
  pub skip_parsing_pdfs: bool,
}

impl Default for UserPreferencesState {
    fn default() -> Self {
        Self {
          first_launch_done: false,
          onboarding_done: false,
          show_search_suggestions: true,
          launch_at_startup: true,
          show_in_dock: true,
          global_shortcut_enabled: true,
          global_shortcut: "Alt+Space".to_string(),
          automatic_background_sync: true,
          detailed_scan: true,
          roadmap_survey_answered: false,
          skip_parsing_pdfs: true
        }
    }
}

use tauri::Wry;
use tauri::menu::Menu;

// Struct for Context Menu
pub(crate) struct ContextMenuState {
  pub folder: Menu<Wry>,
  pub docs: Menu<Wry>,
  pub other: Menu<Wry>,
  pub table_header: Menu<Wry>,
  pub status_bar: Menu<Wry>
}

impl ContextMenuState {
  pub fn new(folder_context_menu:Menu<Wry>, docs_context_menu:Menu<Wry>,
    other_context_menu:Menu<Wry>, table_header_menu:Menu<Wry>, status_bar_menu:Menu<Wry>) -> Self {
    Self {
      folder: folder_context_menu,
      docs: docs_context_menu,
      other: other_context_menu,
      table_header: table_header_menu,
      status_bar: status_bar_menu
    }
  }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryResult {
  pub data: Vec<(i64, String, String, String)>,
  pub is_loading: bool,
  pub error_view: Option<String>,
}
