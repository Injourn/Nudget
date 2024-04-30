use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::models::transaction::Transaction;

const ADD_TRANSACTION: &str = "INSERT INTO transaction_item (amount, category_id, transaction_date,name) VALUES (?1,?2,?3,?4)";

#[tauri::command]
pub(crate) fn add_transaction(conn_state: State<'_, Mutex<Connection>>,transaction:Transaction) {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    add_transaction_sqlite(conn,transaction);
}

pub(crate) fn add_transaction_sqlite(conn: &Connection,transaction:Transaction){
    let execute = conn.execute(ADD_TRANSACTION,
    (&transaction.amount,&transaction.category_id,&transaction.transaction_date,&transaction.name));
    println!("is okay{}", execute.is_ok().to_string());
    if !execute.is_ok() {
        println!("the issue:{}",execute.unwrap_err());
    }

}