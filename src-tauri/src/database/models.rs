// Defines the DocumentItem type
// DocumentSearchResult type is derived from DocumentItem

use diesel::prelude::*;
use diesel::Insertable;
use super::schema::{document, metadata, body};
use serde::Deserialize;
use serde::Serialize;

// This struct is for INSERTING into the document table
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

// This struct is for INSERTING into the metadata table which is something you should NEVER DO
// because metadata table is maintained using triggers in SQLite
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

// This struct is for INSERTING into the body table
#[derive(Serialize, Deserialize, Insertable, Queryable, QueryableByName, PartialEq, Debug, Clone)]
#[diesel(table_name = body)]
pub struct BodyItem {
    pub metadata_id: i32,
    pub text: String,
    pub last_parsed: i64,
}

// This struct is for SELECTING from the metadata table without any JOINs
#[derive(Serialize, Deserialize, Queryable, QueryableByName, PartialEq, Debug, Clone)]
#[diesel(table_name = metadata)]
pub struct MetadataSearchResult {
    pub id: i32,
    pub source_table: String,
    pub source_domain: String,
    pub source_id: i32,
    pub title: String,
    pub url: String,
    pub created_at: i64,
    pub last_modified: i64,
    pub frecency_rank: f32,
    pub frecency_last_accessed: i64,
    // pub comment: Option<String>,
}

// This struct is for SELECTING from the document table without any JOINs
#[derive(Serialize, Deserialize, Queryable, QueryableByName, PartialEq, Debug, Clone)]
#[diesel(table_name = document)]
pub struct DocumentSearchResult {
    pub id: i32,
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

// This struct is for SELECTING from the metadata table with a JOIN to the document table
#[derive(Serialize, Queryable, Debug)]
pub struct DocumentResponseModel {
    pub source_table: String,
    pub source_domain: String,
    pub source_id: i32,
    pub title: String,
    pub url: String,
    pub created_at: i64,
    pub last_modified: i64,
    pub frecency_rank: f32,
    pub frecency_last_accessed: i64,
    pub file_type: String,
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
