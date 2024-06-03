use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::get_all_budget_budget_categories_sqlite, models::{budget::Budget, budget_budget_category::BudgetBudgetCategory}};


#[tauri::command]
pub(crate) fn get_all_budget_budget_categories(conn_state: State<'_, Mutex<Connection>>,budget:Budget) -> Vec<BudgetBudgetCategory>{
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();

    let result = get_all_budget_budget_categories_sqlite(conn,budget).expect("Failed to fetch");

    result
}
