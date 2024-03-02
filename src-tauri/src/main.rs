// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use rusqlite::Connection;

mod commands {
    pub(crate) mod get_listing;
}
mod models {
    pub(crate) mod listing;
    pub(crate) mod category;
    pub(crate) mod sheet_category;
    pub(crate) mod sheet_plan;
    pub(crate) mod cycle;
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let conn = Connection::open("./my_db.db").expect("idk fail?");
    

    tauri::Builder::default()
        .manage(Mutex::new(conn))
        .invoke_handler(tauri::generate_handler![greet,
            crate::commands::get_listing::get_listing])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
