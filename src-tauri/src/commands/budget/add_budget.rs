use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::add_budget_sqlite,
    models::{budget::Budget, response::response::Response},
};

#[tauri::command]
pub(crate) fn add_budget(
    conn_state: State<'_, Mutex<Connection>>,
    budget: Budget,
) -> Response<i64> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    info!("Adding entry to budget.");

    let result = add_budget_sqlite(conn, budget);

    let response = match result {
        Ok(result) => {
            info!("Successfully added budget.");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
