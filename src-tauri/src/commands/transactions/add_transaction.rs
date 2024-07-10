use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::add_transaction_sqlite, models::transaction::Transaction};

#[tauri::command]
pub(crate) fn add_transaction(conn_state: State<'_, Mutex<Connection>>,transaction:Transaction) -> i64 {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    let result = add_transaction_sqlite(conn,transaction).expect("Failed to insert");

    result
}