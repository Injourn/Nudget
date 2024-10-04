// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use rusqlite::Connection;

mod commands {
    pub mod transactions {
        pub(crate) mod get_transactions_in_range;
        pub(crate) mod get_transaction;
        pub(crate) mod get_one_transaction;
        pub(crate) mod add_transaction;
        pub(crate) mod remove_transaction;
        pub(crate) mod update_transaction;
        pub(crate) mod date_recurring_transactions;
    }
    pub mod category {
        pub(crate) mod get_all_categories;
        pub(crate) mod get_one_category;
        pub(crate) mod add_category;
        pub(crate) mod remove_category;
        pub(crate) mod update_category;
    }
    pub mod budget_category{
        pub(crate) mod get_all_budget_categories;
        pub(crate) mod get_one_budget_category;
        pub(crate) mod add_budget_category;
        pub(crate) mod remove_budget_category;
        pub(crate) mod update_budget_category;
    }
    pub mod budget {
        pub(crate) mod get_all_budget;
        pub(crate) mod get_one_budget;
        pub(crate) mod add_budget;
        pub(crate) mod remove_budget;
        pub(crate) mod update_budget;
        pub(crate) mod get_one_budget_by_date;
    }
    pub mod budget_plan{
        pub(crate) mod get_all_budget_plan;
        pub(crate) mod get_one_budget_plan;
        pub(crate) mod get_active_budget_plan;
        pub(crate) mod add_budget_plan;
        pub(crate) mod remove_budget_plan;
        pub(crate) mod update_budget_plan;
    }
    pub mod budget_budget_category{
        pub(crate) mod get_all_budget_budget_categories;
        pub(crate) mod add_budget_budget_category;
        pub(crate) mod remove_budget_budget_category;
    }
    pub mod budget_plan_category{
        pub(crate) mod get_all_budget_plan_categories;
        pub(crate) mod add_budget_plan_category;
        pub(crate) mod remove_budget_plan_category;
    }
    pub(crate) mod get_active_budget_statistics;
    pub(crate) mod get_default_budget_statistics;
    pub(crate) mod load_file;
    pub(crate) mod create_file;
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
    pub(crate) mod account;
    pub mod request{
        pub(crate) mod transaction_in_range_request_model;
    }
    pub mod response{
        pub(crate) mod transaction_response_model;
        pub(crate) mod budget_statistics_response_model;
        pub(crate) mod response;
        pub(crate) mod budget_category_response_model;
    }
}
mod functions {
    pub(crate) mod get_dates_between;
}

mod database {
    pub(crate) mod rusqlite_impl;
    pub mod sql_constants;
}

#[cfg(test)]
mod tests{
    pub mod get_dates_test;
    pub mod sql_tests;
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    
    

    tauri::Builder::default()
        .manage(Mutex::new(conn))
        .invoke_handler(tauri::generate_handler![greet,
            crate::commands::transactions::get_transactions_in_range::get_transactions_in_range,
            crate::commands::transactions::get_one_transaction::get_one_transaction,
            crate::commands::transactions::remove_transaction::remove_transaction,
            crate::commands::transactions::update_transaction::update_transaction,
            crate::commands::transactions::add_transaction::add_transaction,
            crate::commands::transactions::get_transaction::get_transaction,
            crate::commands::category::get_all_categories::get_all_categories,
            crate::commands::category::get_one_category::get_one_category,
            crate::commands::category::add_category::add_category,
            crate::commands::category::remove_category::remove_category,
            crate::commands::category::update_category::update_category,
            crate::commands::budget_category::get_all_budget_categories::get_all_budget_categories,
            crate::commands::budget_category::get_one_budget_category::get_one_budget_category,
            crate::commands::budget_category::add_budget_category::add_budget_category,
            crate::commands::budget_category::remove_budget_category::remove_budget_category,
            crate::commands::budget_category::update_budget_category::update_budget_category,
            crate::commands::budget::get_all_budget::get_all_budget,
            crate::commands::budget::get_one_budget::get_one_budget,
            crate::commands::budget::get_one_budget_by_date::get_one_budget_by_date,
            crate::commands::budget::add_budget::add_budget,
            crate::commands::budget::remove_budget::remove_budget,
            crate::commands::budget::update_budget::update_budget,
            crate::commands::budget_plan::get_all_budget_plan::get_all_budget_plan,
            crate::commands::budget_plan::get_one_budget_plan::get_one_budget_plan,
            crate::commands::budget_plan::get_active_budget_plan::get_active_budget_plan,
            crate::commands::budget_plan::add_budget_plan::add_budget_plan,
            crate::commands::budget_plan::remove_budget_plan::remove_budget_plan,
            crate::commands::budget_plan::update_budget_plan::update_budget_plan,
            crate::commands::budget_budget_category::get_all_budget_budget_categories::get_all_budget_budget_categories,
            crate::commands::budget_budget_category::add_budget_budget_category::add_budget_budget_category,
            crate::commands::budget_budget_category::remove_budget_budget_category::remove_budget_budget_category,
            crate::commands::budget_plan_category::get_all_budget_plan_categories::get_all_budget_plan_categories,
            crate::commands::budget_plan_category::add_budget_plan_category::add_budget_plan_category,
            crate::commands::budget_plan_category::remove_budget_plan_category::remove_budget_plan_category,
            crate::commands::get_default_budget_statistics::get_default_budget_statistics,
            crate::commands::get_active_budget_statistics::get_active_budget_statistics,
            crate::commands::load_file::load_file,
            crate::commands::create_file::create_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
