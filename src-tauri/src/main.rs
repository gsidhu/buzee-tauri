// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate chrono;
extern crate csv;
extern crate diesel;
extern crate dirs;
extern crate dotext;
extern crate epub;
extern crate log;
extern crate mobi;
extern crate pdf_extract;
extern crate rusqlite;
extern crate serde_json;
extern crate tantivy;
extern crate xml;

mod arc_read;
mod chrome_read;
mod context_menu;
mod custom_types;
mod database;
mod db_sync;
mod drag;
mod firefox_read;
mod housekeeping;
mod indexing;
mod ipc;
mod tantivy_index;
mod text_extraction;
mod user_prefs;
mod utils;
mod window;

fn main() {
  housekeeping::initialize();
  ipc::initialize();
}
