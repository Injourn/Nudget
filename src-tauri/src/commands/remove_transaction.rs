use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::models::transaction::Transaction;

const DELETE_TRANSACTION: &str = "DELETE FROM transaction_item where id = ?1";

#[tauri::command]
pub(crate) fn remove_transaction(conn_state: State<'_, Mutex<Connection>>,transaction:Transaction) {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    remove_transaction_sqlite(conn,transaction);
}

pub(crate) fn remove_transaction_sqlite(conn: &Connection,transaction:Transaction){
    let execute = conn.execute(DELETE_TRANSACTION,
    [&transaction.id]);
    println!("is okay{}", execute.is_ok().to_string());
}