use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::add_budget_budget_category_sqlite, models::{budget::Budget, budget_category::BudgetCategory}};


#[tauri::command]
pub(crate) fn add_budget_budget_category(conn_state: State<'_, Mutex<Connection>>,budget_category:BudgetCategory,budget:Budget) {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();

    let result = add_budget_budget_category_sqlite(conn,budget,budget_category);
}