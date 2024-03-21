use serde::{Serialize, Deserialize};

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error)
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
  #[serde(rename = "normalSegments")]
  pub normal_segments: Vec<String>,
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

// Define a struct equivalent to this Typescript interface:
// interface DBStat {
//   type: string,
//   count: number
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct DBStat {
  pub file_type: String,
  pub count: i64
}

// Thread Manager for Tokio sync threads
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct ThreadManager {
  pub handle: Option<JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>>,
  pub kill_thread: bool
}

impl ThreadManager {
  pub fn new() -> Self {
    ThreadManager { 
      handle: None,
      kill_thread: false 
    }
  }
}