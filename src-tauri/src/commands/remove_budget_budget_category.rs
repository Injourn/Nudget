use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::remove_budget_budget_category_sqlite, models::budget_budget_category::BudgetBudgetCategory};


#[tauri::command]
pub(crate) fn remove_budget_budget_category(conn_state: State<'_, Mutex<Connection>>,budget_budget_category:BudgetBudgetCategory) {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    remove_budget_budget_category_sqlite(conn, budget_budget_category);
}
