// Defines the DocumentItem type
// SearchResult type is derived from DocumentItem

use diesel::prelude::*;
use diesel::Insertable;
use super::schema::document;
// use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Insertable, Queryable, QueryableByName, PartialEq, Debug, Clone)]
#[table_name = "document"]
pub struct DocumentItem {
    pub created_at: i64,
    pub name: String,
    pub path: String,
    pub size: Option<f64>,
    pub file_type: String,
    pub file_content: Option<String>,
    pub last_modified: i64,
    pub last_opened: i64,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, QueryableByName, PartialEq, Debug, Clone)]
#[table_name = "document"]
pub struct SearchResult {
    pub created_at: i64,
    pub name: String,
    pub path: String,
    pub size: Option<f64>,
    pub file_type: String,
    pub last_modified: i64,
    pub last_opened: i64,
}

impl DocumentItem {
    pub fn new(
        filename: &str,
        filepath: &str,
        filesize: Option<f64>,
        file_type: &str,
        file_content: Option<String>,
        file_created_at: i64,
        file_last_modified: i64,
        file_last_opened: i64,
    ) -> Self {
        DocumentItem {
            created_at: file_created_at,
            name: filename.to_string(),
            path: filepath.to_string(),
            size: filesize,
            file_type: file_type.to_string(),
            file_content: file_content,
            last_modified: file_last_modified,
            last_opened: file_last_opened
        }
    }
}
