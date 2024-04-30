use rusqlite::{Connection, Params};
use serde::Deserialize;

use crate::models::transaction::{self, Transaction};

const GET_ALL_TRANSACTIONS: &str = "Select id, amount, category_id, transaction_date, name FROM transaction_item";
const GET_ONE_TRANSACTION: &str = "Select id, amount, category_id, transaction_date, name FROM transaction_item where id = ?1";
const ADD_TRANSACTION: &str = "INSERT INTO transaction_item (amount, category_id, transaction_date,name) VALUES (?1,?2,?3,?4)";
const UPDATE_TRANSACTION: &str = "UPDATE transaction_item SET amount = ?2, category_id = ?3, transaction_date = ?4,name = ?5 WHERE transaction_item.id = ?1";
const DELETE_TRANSACTION: &str = "DELETE FROM transaction_item where id = ?1";


pub(crate) fn get_transaction_sqlite(conn: &Connection) -> anyhow::Result<Vec<Transaction>> {
    let result = get_all::<Transaction>(conn,GET_ALL_TRANSACTIONS);

    result
}

pub(crate) fn get_one_transaction_sqlite(conn: &Connection,id: &str) -> anyhow::Result<Transaction>{
    let result = get_one_by_id::<Transaction>(conn, id, GET_ONE_TRANSACTION);

    result
}
pub(crate) fn add_transaction_sqlite(conn: &Connection,transaction: Transaction) -> anyhow::Result<()>{
    let result = insert_or_update_item(conn,
         (&transaction.amount,&transaction.category_id,&transaction.transaction_date,&transaction.name),
        ADD_TRANSACTION);
    
    Ok({})
}

pub(crate) fn update_transaction_sqlite(conn: &Connection,transaction: Transaction) -> anyhow::Result<()>{
    let result = insert_or_update_item(conn,
         (&transaction.id,&transaction.amount,&transaction.category_id,&transaction.transaction_date,&transaction.name),
        UPDATE_TRANSACTION);
    
    Ok({})
}

pub(crate) fn remove_transaction_sqlite(conn: &Connection,transaction:Transaction) -> anyhow::Result<()>{
    let result = remove_item(conn, DELETE_TRANSACTION, transaction.id);

    Ok({})
}



fn remove_item(conn: &Connection,command:&str,id: u32) -> anyhow::Result<()>{
    let execute = conn.execute(command,
        [id]);
    if execute.is_ok() {
        Ok({})
    }
    else {
        Err(execute.unwrap_err().into())
    }
}

fn insert_or_update_item<P:Params>(conn: &Connection,params: P,command: &str) -> anyhow::Result<()>{
    let execute = conn.execute(command,params);
    if execute.is_ok() {
        Ok({})
    }
    else {
        Err(execute.unwrap_err().into())
    }
}

fn get_one_by_id<T: for<'a> Deserialize<'a>>(conn: &Connection,id:&str,command:&str) -> anyhow::Result<T> {
    let parsed_id: u32 = id.parse().unwrap();
    let prepared_stmt = conn.prepare(command);
    if prepared_stmt.is_err() {
        let error_msg = prepared_stmt.unwrap_err();
        println!("failed to prepare statement: {}",error_msg);
        return Err(error_msg.into());
    }
    let mut stmt = prepared_stmt.unwrap();
    let mut rows = stmt.query_map([parsed_id], |row| {
        Ok(serde_rusqlite::from_row::<T>(row).unwrap())
    })?;
    
    let transaction: T = rows.nth(0).unwrap()?;
    rows.last();

    stmt.finalize().unwrap();
    
    Ok(transaction)
}

fn get_all<T: for<'a> Deserialize<'a>>(conn: &Connection,command:&str) -> anyhow::Result<Vec<T>>{
    let prepared_stmt = conn.prepare(command);
    if prepared_stmt.is_err() {
        let error_msg = prepared_stmt.unwrap_err();
        println!("failed to prepare statement: {}",error_msg);
        return Err(error_msg.into());
    }
    let mut stmt = prepared_stmt.unwrap();
    let rows = stmt.query_map([], |row| {
        let result = serde_rusqlite::from_row::<T>(row);
        Ok(result.unwrap())
    })?;
    
    let mut transactions: Vec<T> = Vec::new();

    for transaction in rows {
        transactions.push(transaction?);
    }

    stmt.finalize().unwrap();
    
    Ok(transactions)
}