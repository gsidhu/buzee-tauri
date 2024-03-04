// use crate::error_type::Error; // Import the Error type
use diesel::prelude::*;

const DOCUMENT_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE TABLE IF NOT EXISTS "document" ("id" integer PRIMARY KEY AUTOINCREMENT NOT NULL, "created_at" datetime NOT NULL DEFAULT (datetime('now')), "name" text NOT NULL, "path" text NOT NULL, "size" integer NOT NULL, "type" varchar NOT NULL, "last_modified" datetime NOT NULL, "updated_at" datetime NOT NULL DEFAULT (datetime('now')))
"#;

// const TEXT_TABLE_CREATE_STATEMENT : &str = r#"
//   CREATE TABLE IF NOT EXISTS "text" ("id" integer PRIMARY KEY AUTOINCREMENT NOT NULL, "path" text NOT NULL, "type" text NOT NULL, "last_modified" datetime NOT NULL, "content" text NOT NULL, "document_id" integer NOT NULL, CONSTRAINT "FK_35a9bd82b425cfb0d07a09c3d40" FOREIGN KEY ("document_id") REFERENCES "document" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION)
// "#;

pub fn create_tables_if_not_exists(mut con: diesel::SqliteConnection) -> Result<usize, diesel::result::Error> {
  let exec_query = diesel::sql_query(DOCUMENT_TABLE_CREATE_STATEMENT.to_string());
  let result = exec_query.execute(&mut con)?;
  Ok(result)
}