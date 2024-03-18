// use crate::custom_types::Error;
use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::housekeeping::{get_documents_directory, APP_DIRECTORY};
use queries::{
  DOCUMENT_TABLE_CREATE_STATEMENT, 
  BODY_TABLE_CREATE_STATEMENT, 
  METADATA_TABLE_CREATE_STATEMENT, 
  METADATA_FTS_VIRTUAL_TABLE_CREATE_STATEMENT,
  BODY_FTS_VIRTUAL_TABLE_CREATE_STATEMENT,
  TRIGGER_INSERT_DOCUMENT_METADATA, TRIGGER_UPDATE_DOCUMENT_METADATA, TRIGGER_DELETE_DOCUMENT_METADATA,
  TRIGGER_INSERT_METADATA_FTS, TRIGGER_UPDATE_METADATA_FTS, TRIGGER_DELETE_METADATA_FTS,
  TRIGGER_INSERT_BODY_FTS, TRIGGER_UPDATE_BODY_FTS, TRIGGER_DELETE_BODY_FTS
};

const DB_NAME: &str = r#"buzee.db"#;

// pub mod crud;
pub mod schema;
pub mod models;
pub mod search;
mod queries;
// mod response_models;

pub fn establish_connection() -> SqliteConnection {
  let app_dir = get_documents_directory().unwrap();
  println!("app_dir: {}", app_dir);

  let database_url = format!("sqlite://{}/{}/{}", app_dir, APP_DIRECTORY, DB_NAME);

  SqliteConnection::establish(&database_url).unwrap()
}

// run PRAGMA queries to enable FOREIGN KEYS and timeout
pub fn enable_fts_and_foreign_keys(mut con: diesel::SqliteConnection) -> Result<usize, diesel::result::Error> {
  diesel::sql_query("PRAGMA foreign_keys = ON").execute(&mut con)?;
  diesel::sql_query("PRAGMA busy_timeout = 5000").execute(&mut con)?;
  Ok(1)
}

// Create all tables and triggers in the db if they don't exist
pub fn create_tables_if_not_exists(mut con: diesel::SqliteConnection) -> Result<usize, diesel::result::Error> {
  diesel::sql_query(DOCUMENT_TABLE_CREATE_STATEMENT.to_string()).execute(&mut con)?;
  diesel::sql_query(BODY_TABLE_CREATE_STATEMENT.to_string()).execute(&mut con)?;
  diesel::sql_query(METADATA_TABLE_CREATE_STATEMENT.to_string()).execute(&mut con)?;
  diesel::sql_query(METADATA_FTS_VIRTUAL_TABLE_CREATE_STATEMENT.to_string()).execute(&mut con)?;
  diesel::sql_query(BODY_FTS_VIRTUAL_TABLE_CREATE_STATEMENT.to_string()).execute(&mut con)?;
  diesel::sql_query(TRIGGER_INSERT_DOCUMENT_METADATA.to_string()).execute(&mut con)?;
  diesel::sql_query(TRIGGER_UPDATE_DOCUMENT_METADATA.to_string()).execute(&mut con)?;
  diesel::sql_query(TRIGGER_DELETE_DOCUMENT_METADATA.to_string()).execute(&mut con)?;
  // diesel::sql_query(TRIGGER_INSERT_METADATA_FTS.to_string()).execute(&mut con)?;
  // diesel::sql_query(TRIGGER_UPDATE_METADATA_FTS.to_string()).execute(&mut con)?;
  // diesel::sql_query(TRIGGER_DELETE_METADATA_FTS.to_string()).execute(&mut con)?;
  diesel::sql_query(TRIGGER_INSERT_BODY_FTS.to_string()).execute(&mut con)?;
  diesel::sql_query(TRIGGER_UPDATE_BODY_FTS.to_string()).execute(&mut con)?;
  diesel::sql_query(TRIGGER_DELETE_BODY_FTS.to_string()).execute(&mut con)?;
  Ok(1)
}