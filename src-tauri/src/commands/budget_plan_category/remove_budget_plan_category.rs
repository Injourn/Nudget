use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::remove_budget_plan_category_sqlite, models::budget_plan_category::BudgetPlanCategory};


#[tauri::command]
pub(crate) fn remove_budget_plan_category(conn_state: State<'_, Mutex<Connection>>,budget_plan_category:BudgetPlanCategory) {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    remove_budget_plan_category_sqlite(conn, budget_plan_category);
}
