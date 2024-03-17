use diesel::prelude::*;

table! {
  document (id) {
      id -> Integer,
      source_domain -> Text,
      created_at -> BigInt,
      name -> Text,
      path -> Text,
      size -> Nullable<Double>,
      file_type -> Text,
      last_modified -> BigInt,
      last_opened -> BigInt,
      last_synced -> BigInt,
      is_pinned -> Bool,
      frecency_rank -> Float,
      frecency_last_accessed -> BigInt,
      comment -> Nullable<Text>,
  }
}

table! {
  metadata (id) {
      id -> Integer,
      source_table -> Text,
      source_domain -> Text,
      source_id -> Integer,
      title -> Text,
      url -> Text,
      created_at -> BigInt,
      last_modified -> BigInt,
      frecency_rank -> Float,
      frecency_last_accessed -> BigInt,
      comment -> Nullable<Text>
  }
}

table! {
  body (id) {
      id -> Integer,
      metadata_id -> Integer,
      text -> Text
  }
}