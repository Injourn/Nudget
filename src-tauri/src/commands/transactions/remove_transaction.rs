use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::remove_transaction_sqlite, models::transaction::Transaction};



#[tauri::command]
pub(crate) fn remove_transaction(conn_state: State<'_, Mutex<Connection>>,transaction:Transaction) {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    remove_transaction_sqlite(conn,transaction);
}