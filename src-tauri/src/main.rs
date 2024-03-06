// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;
extern crate chrono;
extern crate serde_json;

mod utils;
mod error_type;
mod ipc;
mod housekeeping;
mod database;
mod indexing;
// mod file_watch;

fn main() {
  housekeeping::initialize();
  ipc::initialize();
}
