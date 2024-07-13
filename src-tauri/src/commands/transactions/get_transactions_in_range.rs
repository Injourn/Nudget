use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::get_transactions_in_range_sqlite, models::{request::transaction_in_range_request_model::TransactionInRangeRequestModel, response::{response::Response, transaction_response_model::TransactionResponseModel}}};


#[tauri::command]
pub(crate) fn get_transactions_in_range(conn_state: State<'_, Mutex<Connection>>,transaction_request:TransactionInRangeRequestModel) -> Response<Vec<TransactionResponseModel>> {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();

    let result = get_transactions_in_range_sqlite(conn,transaction_request);

    let response = match result {
        Ok(result) => Response::success(result),
        Err(error) => Response::error(error),
    };

    response
}