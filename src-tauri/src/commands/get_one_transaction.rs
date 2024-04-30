use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::models::transaction::Transaction;

const GET_ONE_TRANSACTION: &str = "Select id, amount, category_id, transaction_date, name FROM transaction_item where id = ?1";

#[tauri::command]
pub(crate) fn get_one_transaction(conn_state: State<'_, Mutex<Connection>>,id: &str) -> Transaction {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    let result = get_one_transaction_sqlite(conn,id).expect("failed to fetch");

    result
}

fn get_one_transaction_sqlite(conn: &Connection,id:&str) -> anyhow::Result<Transaction> {
    let parsed_id: u32 = id.parse().unwrap();
    let mut stmt = conn.prepare(GET_ONE_TRANSACTION).expect("fail");
    let mut rows = stmt.query_map([parsed_id], |row| {
        Ok(serde_rusqlite::from_row::<Transaction>(row)
            .expect("failed here"))
    })?;
    
    let transaction: Transaction = rows.nth(0).unwrap()?;
    rows.last();

    stmt.finalize().unwrap();
    
    Ok(transaction)
}