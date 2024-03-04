use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::housekeeping::{get_documents_directory, APP_DIRECTORY};
const DB_NAME: &str = r#"buzee.db"#;

pub fn establish_connection() -> SqliteConnection {
  let app_dir = get_documents_directory().unwrap();
  println!("app_dir: {}", app_dir);

  let database_url = format!("sqlite://{}/{}/{}", app_dir, APP_DIRECTORY, DB_NAME);

  SqliteConnection::establish(&database_url)
      .unwrap_or_else(|_| panic!("Error connecting to the database: {}", database_url))
}
