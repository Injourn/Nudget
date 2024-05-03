use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::get_all_budget_categories_sqlite, models::budget_category::BudgetCategory};


#[tauri::command]
pub(crate) fn get_all_budget_categories(conn_state: State<'_, Mutex<Connection>>) -> Vec<BudgetCategory>{
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();

    let result = get_all_budget_categories_sqlite(conn).expect("Failed to fetch");

    result
}
