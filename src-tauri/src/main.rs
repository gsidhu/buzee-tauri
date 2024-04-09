// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate chrono;
extern crate dotext;
extern crate diesel;
extern crate lazy_static;
extern crate log;
extern crate mobi;
extern crate epub;
extern crate regex;
extern crate serde_json;

mod utils;
mod custom_types;
mod ipc;
mod housekeeping;
mod database;
mod db_sync;
mod drag;
mod indexing;
mod text_extraction;
mod context_menu;
mod user_prefs;
mod window;

fn main() {
  housekeeping::initialize();
  ipc::initialize();
}
