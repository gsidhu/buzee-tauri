use diesel::prelude::*;

table! {
  document (id) {
      id -> Integer,
      created_at -> TimestamptzSqlite,
      name -> Text,
      path -> Text,
      size -> Nullable<Double>,
      filetype -> Text,
      content -> Nullable<Text>,
      last_modified -> TimestamptzSqlite,
      updated_at -> TimestamptzSqlite
  }
}