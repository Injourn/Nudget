use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::add_transaction_sqlite,
    models::{response::response::Response, transaction::Transaction},
};

#[tauri::command]
pub(crate) fn add_transaction(
    conn_state: State<'_, Mutex<Connection>>,
    transaction: Transaction,
) -> Response<i64> {
    info!("Adding transaction...");
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    let result = add_transaction_sqlite(conn, transaction);

    let response = match result {
        Ok(result) => {
            info!("Successfully added transaction");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
