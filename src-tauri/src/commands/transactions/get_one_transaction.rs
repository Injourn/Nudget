use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::get_one_transaction_sqlite, models::response::transaction_response_model::TransactionResponseModel};

#[tauri::command]
pub(crate) fn get_one_transaction(conn_state: State<'_, Mutex<Connection>>,id: &str) -> TransactionResponseModel {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    let result: TransactionResponseModel = get_one_transaction_sqlite(conn,id).expect("failed to fetch");
    

    result

}