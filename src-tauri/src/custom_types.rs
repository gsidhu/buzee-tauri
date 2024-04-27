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
#[derive(Serialize, Clone)]
pub(crate) struct UserPreferencesState {
  pub first_launch_done: bool,
  pub onboarding_done: bool,
  pub launch_at_startup: bool,
  pub show_in_dock: bool,
  pub global_shortcut_enabled: bool,
  pub global_shortcut: String,
  pub automatic_background_sync: bool,
  pub detailed_scan: bool
}

impl Default for UserPreferencesState {
    fn default() -> Self {
        Self {
          first_launch_done: false,
          onboarding_done: false,
          launch_at_startup: true,
          show_in_dock: true,
          global_shortcut_enabled: true,
          global_shortcut: "Alt+Space".to_string(),
          automatic_background_sync: true,
          detailed_scan: true
        }
    }
}