use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::get_all_budget_plan_categories_sqlite, models::{budget_category::BudgetCategory, budget_plan::BudgetPlan, response::response::Response}};


#[tauri::command]
pub(crate) fn get_all_budget_plan_categories(conn_state: State<'_, Mutex<Connection>>,budget_plan:BudgetPlan) -> Response<Vec<BudgetCategory>>{
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();

    let result = get_all_budget_plan_categories_sqlite(conn,budget_plan);

    let response = match result {
        Ok(result) => Response::success(result),
        Err(error) => Response::error(error),
    };

    response
}
