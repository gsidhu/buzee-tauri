// use crate::error_type::Error;
use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::housekeeping::{get_documents_directory, APP_DIRECTORY};
use queries::{DOCUMENT_TABLE_CREATE_STATEMENT, CREATE_DOCUMENT_FTS, TRIGGER_DOCUMENT_INSERT, TRIGGER_DOCUMENT_UPDATE, TRIGGER_DOCUMENT_DELETE};

const DB_NAME: &str = r#"buzee.db"#;

// pub mod crud;
pub mod schema;
pub mod models;
pub mod search;
mod queries;

pub fn establish_connection() -> SqliteConnection {
  let app_dir = get_documents_directory().unwrap();
  println!("app_dir: {}", app_dir);

  let database_url = format!("sqlite://{}/{}/{}", app_dir, APP_DIRECTORY, DB_NAME);

  SqliteConnection::establish(&database_url).unwrap()
}

// Create all tables and triggers in the db if they don't exist
pub fn create_tables_if_not_exists(mut con: diesel::SqliteConnection) -> Result<usize, diesel::result::Error> {
  diesel::sql_query(DOCUMENT_TABLE_CREATE_STATEMENT.to_string()).execute(&mut con)?;
  diesel::sql_query(CREATE_DOCUMENT_FTS.to_string()).execute(&mut con)?;
  diesel::sql_query(TRIGGER_DOCUMENT_INSERT.to_string()).execute(&mut con)?;
  diesel::sql_query(TRIGGER_DOCUMENT_UPDATE.to_string()).execute(&mut con)?;
  diesel::sql_query(TRIGGER_DOCUMENT_DELETE.to_string()).execute(&mut con)?;
  Ok(1)
}