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
#[derive(Debug, Serialize, Deserialize)]
pub struct DateLimit {
  pub start: String,
  pub end: String,
  pub text: String
}

// Payload for IPC events
#[derive(Clone, serde::Serialize)]
pub struct Payload {
  pub message: String,
}