use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::add_budget_plan_category_sqlite, models::{budget_category::BudgetCategory, budget_plan::BudgetPlan}};


#[tauri::command]
pub(crate) fn add_budget_plan_category(conn_state: State<'_, Mutex<Connection>>,budget_category:BudgetCategory,budget_plan:BudgetPlan) {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();

    let result = add_budget_plan_category_sqlite(conn,budget_plan,budget_category);
}