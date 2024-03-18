// Defines the DocumentItem type
// SearchResult type is derived from DocumentItem

use diesel::prelude::*;
use diesel::Insertable;
use super::schema::{document, metadata};
// use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Insertable, Queryable, QueryableByName, PartialEq, Debug, Clone)]
#[diesel(table_name = document)]
pub struct DocumentItem {
    pub source_domain: String,
    pub created_at: i64,
    pub name: String,
    pub path: String,
    pub size: Option<f64>,
    pub file_type: String,
    pub last_modified: i64,
    pub last_opened: i64,
    pub last_synced: i64,
    pub is_pinned: bool,
    pub frecency_rank: f32,
    pub frecency_last_accessed: i64,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, QueryableByName, PartialEq, Debug, Clone)]
#[diesel(table_name = metadata)]
pub struct MetadataItem {
    pub source_table: String,
    pub source_domain: String,
    pub source_id: i32,
    pub title: String,
    pub url: String,
    pub created_at: i64,
    pub last_modified: i64,
    pub frecency_rank: f32,
    pub frecency_last_accessed: i64,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, QueryableByName, PartialEq, Debug, Clone)]
#[diesel(table_name = metadata)]
pub struct SearchResult {
    // pub id: i32,
    // pub source_table: String,
    // pub source_domain: String,
    // pub source_id: i32,
    pub title: String,
    pub url: String,
    pub created_at: i64,
    pub last_modified: i64,
    // pub frecency_rank: f32,
    // pub frecency_last_accessed: i64,
    // pub comment: Option<String>,
}

// impl DocumentItem {
//     pub fn new(
//         source_domain: &str,
//         file_created_at: i64,
//         filename: &str,
//         filepath: &str,
//         filesize: Option<f64>,
//         file_type: &str,
//         file_last_modified: i64,
//         file_last_opened: i64,
//         file_last_synced: i64,
//         is_pinned: bool,
//         frecency_rank: f32,
//         frecency_last_accessed: i64,
//         comment: Option<String>,
//         metadata_id: i32,
//     ) -> Self {
//         DocumentItem {
//             source_domain: source_domain.to_string(),
//             created_at: file_created_at,
//             name: filename.to_string(),
//             path: filepath.to_string(),
//             size: filesize,
//             file_type: file_type.to_string(),
//             last_modified: file_last_modified,
//             last_opened: file_last_opened,
//             last_synced: file_last_synced,
//             is_pinned,
//             frecency_rank,
//             frecency_last_accessed,
//             comment,
//             metadata_id,
//         }
//     }
// }
