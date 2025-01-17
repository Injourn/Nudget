use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::update_budget_plan_sqlite,
    models::{budget_plan::BudgetPlan, response::response::Response},
};

#[tauri::command]
pub(crate) fn update_budget_plan(
    conn_state: State<'_, Mutex<Connection>>,
    budget_plan: BudgetPlan,
) -> Response<i64> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    let result = update_budget_plan_sqlite(conn, budget_plan);

    let response = match result {
        Ok(result) => Response::success(result),
        Err(error) => Response::error(error),
    };

    response
}
