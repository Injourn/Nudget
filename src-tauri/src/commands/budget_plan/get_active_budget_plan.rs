use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::get_active_budget_plan_sqlite,
    models::{budget_plan::BudgetPlan, response::response::Response},
};

#[tauri::command]
pub(crate) fn get_active_budget_plan(
    conn_state: State<'_, Mutex<Connection>>,
) -> Response<Option<BudgetPlan>> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    info!("Getting active budget plan");

    let result = get_active_budget_plan_sqlite(conn);

    let response = match result {
        Ok(result) => {
            info!("Successfully got active budget plan.");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
