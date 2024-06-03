// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use rusqlite::Connection;

mod commands {
    pub(crate) mod get_transaction;
    pub(crate) mod get_one_transaction;
    pub(crate) mod add_transaction;
    pub(crate) mod remove_transaction;
    pub(crate) mod update_transaction;
    pub(crate) mod get_all_categories;
    pub(crate) mod get_one_category;
    pub(crate) mod add_category;
    pub(crate) mod remove_category;
    pub(crate) mod update_category;
    pub(crate) mod get_all_budget_categories;
    pub(crate) mod get_one_budget_category;
    pub(crate) mod add_budget_category;
    pub(crate) mod remove_budget_category;
    pub(crate) mod update_budget_category;
    pub(crate) mod get_all_budget;
    pub(crate) mod get_one_budget;
    pub(crate) mod add_budget;
    pub(crate) mod remove_budget;
    pub(crate) mod update_budget;
    pub(crate) mod get_all_budget_plan;
    pub(crate) mod get_one_budget_plan;
    pub(crate) mod add_budget_plan;
    pub(crate) mod remove_budget_plan;
    pub(crate) mod update_budget_plan;
    pub(crate) mod get_all_budget_budget_categories;
    pub(crate) mod add_budget_budget_category;
    pub(crate) mod remove_budget_budget_category;
    pub(crate) mod get_all_budget_plan_categories;
    pub(crate) mod add_budget_plan_category;
    pub(crate) mod remove_budget_plan_category;
}
mod models {
    pub(crate) mod transaction;
    pub(crate) mod category;
    pub(crate) mod budget_category;
    pub(crate) mod budget_plan;
    pub(crate) mod budget;
    pub(crate) mod cycle;
    pub(crate) mod budget_plan_category;
    pub(crate) mod budget_budget_category;
    pub mod response{
        pub(crate) mod transaction_response_model;
    }
}

mod database {
    pub(crate) mod rusqlite_impl;
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let conn = Connection::open("../my_db.db").expect("idk fail?");
    

    tauri::Builder::default()
        .manage(Mutex::new(conn))
        .invoke_handler(tauri::generate_handler![greet,
            crate::commands::get_all_categories::get_all_categories,
            crate::commands::get_one_category::get_one_category,
            crate::commands::add_category::add_category,
            crate::commands::remove_category::remove_category,
            crate::commands::update_category::update_category,
            crate::commands::get_transaction::get_transaction,
            crate::commands::get_all_budget_categories::get_all_budget_categories,
            crate::commands::get_one_budget_category::get_one_budget_category,
            crate::commands::add_budget_category::add_budget_category,
            crate::commands::remove_budget_category::remove_budget_category,
            crate::commands::update_budget_category::update_budget_category,
            crate::commands::get_all_budget::get_all_budget,
            crate::commands::get_one_budget::get_one_budget,
            crate::commands::add_budget::add_budget,
            crate::commands::remove_budget::remove_budget,
            crate::commands::update_budget::update_budget,
            crate::commands::get_all_budget_plan::get_all_budget_plan,
            crate::commands::get_one_budget_plan::get_one_budget_plan,
            crate::commands::add_budget_plan::add_budget_plan,
            crate::commands::get_all_budget_budget_categories::get_all_budget_budget_categories,
            crate::commands::add_budget_budget_category::add_budget_budget_category,
            crate::commands::remove_budget_budget_category::remove_budget_budget_category,
            crate::commands::get_all_budget_plan_categories::get_all_budget_plan_categories,
            crate::commands::add_budget_plan_category::add_budget_plan_category,
            crate::commands::remove_budget_plan_category::remove_budget_plan_category,
            crate::commands::remove_budget_plan::remove_budget_plan,
            crate::commands::update_budget_plan::update_budget_plan,
            crate::commands::get_one_transaction::get_one_transaction,
            crate::commands::remove_transaction::remove_transaction,
            crate::commands::update_transaction::update_transaction,
            crate::commands::add_transaction::add_transaction])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
