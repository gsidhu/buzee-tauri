use diesel::prelude::*;

table! {
  document (id) {
      id -> Integer,
      created_at -> BigInt,
      name -> Text,
      path -> Text,
      size -> Nullable<Double>,
      file_type -> Text,
      file_content -> Nullable<Text>,
      last_modified -> BigInt,
      last_opened -> BigInt
  }
}