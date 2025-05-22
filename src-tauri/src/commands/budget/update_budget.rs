use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::update_budget_sqlite,
    models::{budget::Budget, response::response::Response},
};

#[tauri::command]
pub(crate) fn update_budget(
    conn_state: State<'_, Mutex<Connection>>,
    budget: Budget,
) -> Response<i64> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    info!("Updating budget");
    let conn = conn.deref();
    let result = update_budget_sqlite(conn, budget);

    let response = match result {
        Ok(result) => {
            info!("Successfully updated budget.");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
