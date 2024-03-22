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
  TRIGGER_INSERT_BODY_FTS, TRIGGER_UPDATE_BODY_FTS, TRIGGER_DELETE_BODY_FTS,
  USER_PREFS_TABLE_CREATE_STATEMENT,
  APP_DATA_TABLE_CREATE_STATEMENT,
  FILE_TYPES_TABLE_CREATE_STATEMENT
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
  let mut connection = SqliteConnection::establish(&database_url).unwrap();

  // run PRAGMA queries on each connection
  diesel::sql_query("PRAGMA foreign_keys = ON;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA busy_timeout = 5000;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA journal_mode = WAL;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA synchronous = NORMAL;").execute(&mut connection).unwrap();

  connection
}

// Create all tables and triggers in the db if they don't exist
pub fn create_tables_if_not_exists(conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
  // User Prefs and App Data Tables
  diesel::sql_query(USER_PREFS_TABLE_CREATE_STATEMENT.to_string()).execute(conn)?;
  diesel::sql_query(APP_DATA_TABLE_CREATE_STATEMENT.to_string()).execute(conn)?;
  diesel::sql_query(FILE_TYPES_TABLE_CREATE_STATEMENT.to_string()).execute(conn)?;
  
  // Data Tables
  diesel::sql_query(DOCUMENT_TABLE_CREATE_STATEMENT.to_string()).execute(conn)?;
  diesel::sql_query(BODY_TABLE_CREATE_STATEMENT.to_string()).execute(conn)?;
  diesel::sql_query(METADATA_TABLE_CREATE_STATEMENT.to_string()).execute(conn)?;
  diesel::sql_query(METADATA_FTS_VIRTUAL_TABLE_CREATE_STATEMENT.to_string()).execute(conn)?;
  diesel::sql_query(BODY_FTS_VIRTUAL_TABLE_CREATE_STATEMENT.to_string()).execute(conn)?;

  // Triggers
  diesel::sql_query(TRIGGER_INSERT_DOCUMENT_METADATA.to_string()).execute(conn)?;
  diesel::sql_query(TRIGGER_UPDATE_DOCUMENT_METADATA.to_string()).execute(conn)?;
  diesel::sql_query(TRIGGER_DELETE_DOCUMENT_METADATA.to_string()).execute(conn)?;
  diesel::sql_query(TRIGGER_INSERT_BODY_FTS.to_string()).execute(conn)?;
  diesel::sql_query(TRIGGER_UPDATE_BODY_FTS.to_string()).execute(conn)?;
  diesel::sql_query(TRIGGER_DELETE_BODY_FTS.to_string()).execute(conn)?;
  Ok(1)
}