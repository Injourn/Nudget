use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::models::transaction::Transaction;

#[tauri::command]
pub(crate) fn get_transaction(conn_state: State<'_, Mutex<Connection>>) -> Vec<Transaction> {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    let result = get_transaction_sqlite(conn).expect("failed to fetch");

    result
}

fn get_transaction_sqlite(conn: &Connection) -> anyhow::Result<Vec<Transaction>> {
    let mut stmt = conn.prepare("Select id, amount, category, date, name FROM transaction").expect("fail");
    let rows = stmt.query_map([], |row| {
        Ok(serde_rusqlite::from_row::<Transaction>(row)
            .expect("failed here"))
    })?;
    
    let mut transactions: Vec<Transaction> = Vec::new();

    for transaction in rows {
        transactions.push(transaction?);
    }

    stmt.finalize().unwrap();
    
    Ok(transactions)
}