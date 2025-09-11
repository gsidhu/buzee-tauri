// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate chrono;
extern crate dotext;
extern crate diesel;
extern crate futures;
extern crate jfs;
extern crate lazy_static;
extern crate log;
extern crate mobi;
extern crate epub;
extern crate pdf_extract;
extern crate regex;
extern crate reqwest;
extern crate serde_json;
extern crate tantivy;
extern crate xml;
extern crate zip;
extern crate csv;
extern crate rusqlite;
extern crate dirs;

mod utils;
mod custom_types;
mod ipc;
mod housekeeping;
mod database;
mod db_sync;
// mod drag;
mod indexing;
mod text_extraction;
mod context_menu;
mod user_prefs;
mod tantivy_index;
mod window;
mod firefox_read;
mod chrome_read;
mod arc_read;

fn main() {
  housekeeping::initialize();
  ipc::initialize();
}
