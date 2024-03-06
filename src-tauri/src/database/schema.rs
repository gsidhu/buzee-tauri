use diesel::prelude::*;

table! {
  document (id) {
      id -> Integer,
      created_at -> Text,
      name -> Text,
      path -> Text,
      size -> Nullable<Double>,
      file_type -> Text,
      content -> Nullable<Text>,
      last_modified -> Text,
      last_opened -> Text
  }
}