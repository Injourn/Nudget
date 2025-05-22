use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::get_all_budget_sqlite,
    models::{budget::Budget, response::response::Response},
};

#[tauri::command]
pub(crate) fn get_all_budget(conn_state: State<'_, Mutex<Connection>>) -> Response<Vec<Budget>> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();

    info!("Getting all budget objects");
    let result = get_all_budget_sqlite(conn);

    let response = match result {
        Ok(result) => {
            info!("Successfully got all budgets.");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
