use rusqlite::{Connection, Params};
use serde::Deserialize;

use crate::models::{category::Category, response::transaction_response_model::TransactionResponseModel, transaction::{self, Transaction}};

const GET_ALL_TRANSACTIONS: &str = "Select transaction_item.id, amount,c.id as category_id, c.name as category_name, transaction_date, transaction_item.name FROM transaction_item join category c on c.id = transaction_item.category_id";
const GET_ONE_TRANSACTION: &str = "Select transaction_item.id, amount,c.id as category_id, c.name as category_name, transaction_date, transaction_item.name FROM transaction_item join category c on c.id = transaction_item.category_id where id = ?1";
const ADD_TRANSACTION: &str = "INSERT INTO transaction_item (amount, category_id, transaction_date,name) VALUES (?1,?2,?3,?4)";
const UPDATE_TRANSACTION: &str = "UPDATE transaction_item SET amount = ?2, category_id = ?3, transaction_date = ?4,name = ?5 WHERE transaction_item.id = ?1";
const DELETE_TRANSACTION: &str = "DELETE FROM transaction_item where id = ?1";

const INSERT_CATEGORY: &str = "INSERT INTO category (name) VALUES (?1)";
const UPDATE_CATEGORY: &str = "UPDATE category SET name = ?2 WHERE category.id = ?1";
const GET_ALL_CATEGORIES: &str = "SELECT id, name FROM category";
const GET_ONE_CATEGORY: &str = "SELECT id, name FROM category WHERE category.id = ?1";
const DELETE_CATEGORY: &str = "DELETE FROM category WHERE id = ?1";


pub(crate) fn get_transaction_sqlite(conn: &Connection) -> anyhow::Result<Vec<TransactionResponseModel>> {
    let result = get_all::<TransactionResponseModel>(conn,GET_ALL_TRANSACTIONS);

    result
}

pub(crate) fn get_one_transaction_sqlite(conn: &Connection,id: &str) -> anyhow::Result<TransactionResponseModel>{
    let result = get_one_by_id::<TransactionResponseModel>(conn, id, GET_ONE_TRANSACTION);

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

pub(crate) fn add_category_sqlite(conn: &Connection,category:Category) -> anyhow::Result<()>{
    let result = insert_or_update_item(conn, [category.name], INSERT_CATEGORY);

    Ok({})
}

pub(crate) fn update_category_sqlite(conn: &Connection,category: Category) -> anyhow::Result<()>{
    let result = insert_or_update_item(conn,
         (&category.id,&category.name),
        UPDATE_CATEGORY);
    
    Ok({})
}

pub(crate) fn remove_category_sqlite(conn: &Connection,category: Category) -> anyhow::Result<()>{
    let result = remove_item(conn, DELETE_CATEGORY, category.id);

    Ok({})
}

pub(crate) fn get_all_categories_sqlite(conn: &Connection) -> anyhow::Result<Vec<Category>> {
    let result = get_all::<Category>(conn,GET_ALL_CATEGORIES);

    result
}

pub(crate) fn get_one_category_sqlite(conn: &Connection,id: &str) -> anyhow::Result<Category>{
    let result = get_one_by_id::<Category>(conn, id, GET_ONE_CATEGORY);

    result
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
        let error = execute.unwrap_err();
        println!("Error: {}",error);
        Err(error.into())
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