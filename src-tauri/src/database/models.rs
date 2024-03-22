// Defines the DocumentItem type
// DocumentSearchResult type is derived from DocumentItem

use diesel::prelude::*;
use diesel::Insertable;
use super::schema::{document, metadata, body, user_preferences, app_data, file_types};
use serde::Deserialize;
use serde::Serialize;

// This struct is for CRUD on the user_prefs table
#[derive(Serialize, Deserialize, Insertable, Queryable, QueryableByName, PartialEq, Debug, Clone)]
#[diesel(table_name = user_preferences)]
pub struct UserPrefs {
    pub first_launch_done: bool,
    pub onboarding_done: bool,
    pub launch_at_startup: bool,
    pub show_in_dock: bool,
    pub global_shortcut_enabled: bool,
    pub global_shortcut: String,
    pub automatic_background_sync: bool,
    pub detailed_scan: bool,
    pub disallowed_paths: String,
}

// This struct is for CRUD on the app_data table
#[derive(Serialize, Deserialize, Insertable, Queryable, QueryableByName, PartialEq, Debug, Clone)]
#[diesel(table_name = app_data)]
pub struct AppData {
    pub app_name: String,
    pub app_version: String,
    pub app_mode: String,
    pub app_theme: String,
    pub app_language: String,
    pub last_scan_time: i64,
    pub scan_running: bool
}

// This struct is for CRUD on the file_types table
#[derive(Serialize, Deserialize, Insertable, Queryable, QueryableByName, PartialEq, Debug, Clone)]
#[diesel(table_name = file_types)]
pub struct FileTypes {
    pub file_type: String,
    pub file_type_category: String,
    pub file_type_allowed: bool,
    pub added_by_user: bool,
}

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