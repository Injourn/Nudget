use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::get_transaction_sqlite, models::transaction::Transaction};

#[tauri::command]
pub(crate) fn get_transaction(conn_state: State<'_, Mutex<Connection>>) -> Vec<Transaction> {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    let result = get_transaction_sqlite(conn).expect("failed to fetch");

    result
}