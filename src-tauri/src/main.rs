// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error_type;
mod ipc;
mod housekeeping;
mod database;
mod schema;

fn main() {
  housekeeping::initialize();
  ipc::initialize();
}
