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
      last_parsed -> BigInt,
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
      comment -> Nullable<Text>,
      extra_tag -> Text
  }
}

table! {
  metadata_fts (id) {
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
      comment -> Nullable<Text>,
      extra_tag -> Text
  }
}

table! {
  body (id) {
      id -> Integer,
      metadata_id -> Integer,
      source_id -> Integer,
      text -> Text,
      last_parsed -> BigInt,
  }
}

table! {
  body_fts (metadata_id) {
      metadata_id -> Integer,
      text -> Text,
      title -> Text,
      url -> Text,
      last_parsed -> BigInt,
  }
}

joinable!(body -> metadata (metadata_id));
joinable!(document -> metadata (id));

allow_tables_to_appear_in_same_query!(
  document,
  metadata,
  body
);

table! {
  user_preferences (id) {
    id -> Integer,
    first_launch_done -> Bool,
    onboarding_done -> Bool,
    show_search_suggestions -> Bool,
    launch_at_startup -> Bool,
    show_in_dock -> Bool,
    global_shortcut_enabled -> Bool,
    global_shortcut -> Text,
    automatic_background_sync -> Bool,
    detailed_scan -> Bool,
    roadmap_survey_answered -> Bool,
    skip_parsing_pdfs -> Bool,
    manual_setup -> Bool
  }
}

table! {
  app_data (id) {
    id -> Integer,
    app_name -> Text,
    app_version -> Text,
    app_mode -> Text,
    app_theme -> Text,
    app_language -> Text,
    last_scan_time -> BigInt,
    scan_running -> Bool
  }
}

table! {
  ignore_list (id) {
    id -> Integer,
    path -> Text,
    is_folder -> Bool,
    ignore_indexing -> Bool
  }
}

table! {
  allow_list (id) {
    id -> Integer,
    path -> Text,
    is_folder -> Bool
  }
}

table! {
  file_types (id) {
    id -> Integer,
    file_type -> Text,
    file_type_category -> Text,
    file_type_allowed -> Bool,
    added_by_user -> Bool
  }
}