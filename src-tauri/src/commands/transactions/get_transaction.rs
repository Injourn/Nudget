use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::get_transaction_sqlite,
    models::response::{response::Response, transaction_response_model::TransactionResponseModel},
};

#[tauri::command]
pub(crate) fn get_transaction(
    conn_state: State<'_, Mutex<Connection>>,
) -> Response<Vec<TransactionResponseModel>> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    let result = get_transaction_sqlite(conn);

    let response = match result {
        Ok(result) => Response::success(result),
        Err(error) => Response::error(error),
    };

    response
}
