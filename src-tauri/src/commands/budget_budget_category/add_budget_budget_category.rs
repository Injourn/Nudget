use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::add_budget_budget_category_sqlite,
    models::response::response::Response,
};

#[tauri::command]
pub(crate) fn add_budget_budget_category(
    conn_state: State<'_, Mutex<Connection>>,
    budget_category_id: u32,
    budget_id: u32,
) -> Response<i64> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    info!("Adding budget budget category");

    let result = add_budget_budget_category_sqlite(conn, budget_id, budget_category_id);

    let response = match result {
        Ok(result) => {
            info!("Successfully added budget budget category");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
