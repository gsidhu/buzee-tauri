use std::sync::Mutex;
use tauri::Manager;
use crate::custom_types::DBConnPoolState;
// use crate::custom_types::Error;
use diesel::prelude::*;
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager};
use diesel::SqliteConnection;
use crate::housekeeping::{get_documents_directory, APP_DIRECTORY};
use crate::utils::norm;
use queries::{
  DOCUMENT_TABLE_CREATE_STATEMENT, 
  BODY_TABLE_CREATE_STATEMENT, 
  METADATA_TABLE_CREATE_STATEMENT, 
  METADATA_FTS_VIRTUAL_TABLE_CREATE_STATEMENT,
  BODY_FTS_VIRTUAL_TABLE_CREATE_STATEMENT,
  TRIGGER_INSERT_DOCUMENT_METADATA, TRIGGER_UPDATE_DOCUMENT_METADATA,
  TRIGGER_INSERT_BODY_FTS, TRIGGER_UPDATE_BODY_FTS,
  USER_PREFS_TABLE_CREATE_STATEMENT,
  USER_PREFS_TABLE_ALTER_STATEMENT_V_0_2_0,
  APP_DATA_TABLE_CREATE_STATEMENT,
  IGNORE_LIST_TABLE_CREATE_STATEMENT,
  ALLOW_LIST_TABLE_CREATE_STATEMENT,
  FILE_TYPES_TABLE_CREATE_STATEMENT
};

const DB_NAME: &str = r#"buzee.db"#;

// pub mod crud;
pub mod schema;
pub mod models;
pub mod search;
mod queries;
// mod response_models;

fn get_db_url() -> String {
  let app_dir = get_documents_directory().unwrap();
  println!("app_dir: {}", app_dir);
  let database_path = format!("{}/{}/{}", app_dir, APP_DIRECTORY, DB_NAME);
  let database_path = norm(&database_path);
  let database_url: String;
  #[cfg(target_os = "windows")]
  {
    database_url = format!("sqlite:///{}", database_path);
  }
  #[cfg(target_os = "macos")]
  {
    database_url = format!("sqlite://{}", database_path);
  }
  database_url
}

pub fn get_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
  let database_url = get_db_url();
  println!("Creating connection pool for db at: {}", &database_url);
  let manager = ConnectionManager::<SqliteConnection>::new(database_url);
  Pool::builder()
      .test_on_check_out(true)
      .max_size(10)
      .build(manager)
      .expect("Could not build connection pool")
}

pub fn establish_connection(app: &tauri::AppHandle) -> PooledConnection<ConnectionManager<SqliteConnection>> {
  let state_mutex = app.state::<Mutex<DBConnPoolState>>();
  let state = state_mutex.lock().unwrap();
  let pool = &state.conn_pool;

  println!("Getting db connection from pool");
  let mut connection = pool.get().unwrap();

  // run PRAGMA queries on each connection
  diesel::sql_query("PRAGMA foreign_keys = ON;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA busy_timeout = 5000;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA journal_mode = WAL;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA cache_size = 1000000000;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA synchronous = NORMAL;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA auto_vacuum = FULL;").execute(&mut connection).unwrap();

  connection
}

pub fn establish_direct_connection_to_db() -> SqliteConnection {
  let database_url = get_db_url();
  println!("Creating direct connection to db at: {}", &database_url);
  let mut connection = SqliteConnection::establish(&database_url).unwrap();

  // run PRAGMA queries on each connection
  diesel::sql_query("PRAGMA foreign_keys = ON;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA busy_timeout = 5000;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA journal_mode = WAL;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA cache_size = 1000000000;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA synchronous = NORMAL;").execute(&mut connection).unwrap();
  diesel::sql_query("PRAGMA auto_vacuum = FULL;").execute(&mut connection).unwrap();

  connection
}

// Create all tables and triggers in the db if they don't exist
pub fn create_tables_if_not_exists(conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
  let user_prefs_columns = get_table_columns(conn, "user_preferences")?;
  if !user_prefs_columns.contains(&"roadmap_survey_answered".to_string()) {
    diesel::sql_query(USER_PREFS_TABLE_ALTER_STATEMENT_V_0_2_0.to_string()).execute(conn)?;
  }

  // User Prefs and App Data Tables
  diesel::sql_query(USER_PREFS_TABLE_CREATE_STATEMENT.to_string()).execute(conn)?;
  diesel::sql_query(APP_DATA_TABLE_CREATE_STATEMENT.to_string()).execute(conn)?;
  diesel::sql_query(ALLOW_LIST_TABLE_CREATE_STATEMENT.to_string()).execute(conn)?;
  diesel::sql_query(IGNORE_LIST_TABLE_CREATE_STATEMENT.to_string()).execute(conn)?;
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
  diesel::sql_query(TRIGGER_INSERT_BODY_FTS.to_string()).execute(conn)?;
  diesel::sql_query(TRIGGER_UPDATE_BODY_FTS.to_string()).execute(conn)?;
  Ok(1)
}

use diesel::sql_query;

#[derive(QueryableByName, Debug)]
struct TableInfo {
    #[sql_type = "diesel::sql_types::Text"]
    name: String
}

fn get_table_columns(conn: &mut SqliteConnection, table_name: &str) -> Result<Vec<String>, diesel::result::Error> {
    let query = format!("SELECT name FROM pragma_table_info('{}')", table_name);
    let results = sql_query(query).load::<TableInfo>(conn)?;

    let mut columns = Vec::new();
    for result in results {
        columns.push(result.name);
    }
    Ok(columns)
}