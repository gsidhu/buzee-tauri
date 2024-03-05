// Defines the DocumentItem type
// SearchResult type is derived from DocumentItem
use chrono::{Local, NaiveDateTime};

#[derive(PartialEq, Debug)]
pub struct DocumentItem {
  pub id: i32,
  pub created_at: NaiveDateTime,
  pub name: String,
  pub path: String,
  pub size: Option<f64>,
  pub filetype: String,
  pub content: Option<String>,
  pub last_modified: NaiveDateTime,
  pub updated_at: NaiveDateTime
}

#[derive(PartialEq, Debug)]
pub struct SearchResult {
  pub file_view: Vec<DocumentItem>,
  pub tokenized: String,
}

// impl DocumentItem {
//   fn new() -> Self {
//       DocumentItem {
//         id: 0,
//         created_at: Local::now().naive_local(),
//         name: "".to_string(),
//         path: "".to_string(),
//         size: None,
//         filetype: "".to_string(),
//         content: None,
//         last_modified: Local::now().naive_local(),
//         updated_at: Local::now().naive_local()
//     }
//   }
// }