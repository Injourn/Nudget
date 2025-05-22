use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::remove_transaction_sqlite,
    models::{response::response::Response, transaction::Transaction},
};

#[tauri::command]
pub(crate) fn remove_transaction(
    conn_state: State<'_, Mutex<Connection>>,
    transaction: Transaction,
) -> Response<()> {
    info!("Removing a single transaction.");
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    let result = remove_transaction_sqlite(conn, transaction);

    let response = match result {
        Ok(_) => {
            info!("Successfully removed transaction");
            Response::success(())
        },
        Err(error) => Response::error(error),
    };

    response
}
