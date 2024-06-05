use rusqlite::{Connection, Params};
use serde::Deserialize;

use crate::models::{budget::Budget, budget_budget_category::BudgetBudgetCategory, budget_category::{self, BudgetCategory}, budget_plan::BudgetPlan, budget_plan_category::BudgetPlanCategory, category::Category, response::{budget_statistics_response_model::BudgetStatisticsResponseModel, transaction_response_model::TransactionResponseModel}, transaction::{self, Transaction}};

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

const INSERT_BUDGET_CATEGORY: &str = "INSERT INTO budget_category (category_id,flat_amount,percentage_amount,fixed) VALUES (?1,?2,?3,?4)";
const UPDATE_BUDGET_CATEGORY: &str = "UPDATE budget_category SET category_id = ?2,flat_amount = ?3,percentage_amount = ?4, fixed = ?5 WHERE category.id = ?1";
const GET_ALL_BUDGET_CATEGORIES: &str = "SELECT id,category_id,flat_amount,percentage_amount, fixed FROM budget_category";
const GET_ONE_BUDGET_CATEGORY: &str = "SELECT id,category_id,flat_amount,percentage_amount, fixed FROM budget_category WHERE category.id = ?1";
const DELETE_BUDGET_CATEGORY: &str = "DELETE FROM budget_category WHERE id = ?1";

const INSERT_BUDGET: &str = "INSERT INTO budget (start_date,cycle) VALUES (?1,?2)";
const UPDATE_BUDGET: &str = "UPDATE budget SET start_date = ?2,cycle = ?3 WHERE budget.id = ?1";
const GET_ALL_BUDGET: &str = "SELECT id,start_date,cycle FROM budget";
const GET_ONE_BUDGET: &str = "SELECT id,start_date,cycle FROM budget WHERE budget.id = ?1";
const DELETE_BUDGET: &str = "DELETE FROM budget WHERE id = ?1";

const INSERT_BUDGET_PLAN: &str = "INSERT INTO budget_plan (cycle) VALUES (?1)";
const UPDATE_BUDGET_PLAN: &str = "UPDATE budget_plan SET cycle = ?3 WHERE budget_plan.id = ?1";
const GET_ALL_BUDGET_PLAN: &str = "SELECT id,cycle FROM budget_plan";
const GET_ONE_BUDGET_PLAN: &str = "SELECT id,cycle FROM budget_plan WHERE budget_plan.id = ?1";
const DELETE_BUDGET_PLAN: &str = "DELETE FROM budget_plan WHERE id = ?1";

const INSERT_BUDGET_BUDGET_CATEGORIES: &str = "INSERT INTO budget_budget_category (budget_category_id,budget_plan_id) VALUES (?1,?2)";
const GET_ALL_BUDGET_BUDGET_CATEGORIES: &str = "SELECT bc.id,bc.category_id,bc.flat_amount,bc.percentage_amount FROM budget_budget_category bbc JOIN budget_category bc ON bc.id = bbc.budget_category_id where bbc.budget_id = ?1";
const DELETE_BUDGET_BUDGET_CATEGORY: &str = "DELETE FROM budget_budget_category WHERE budget_category_id = ?1 AND budget_plan_id = ?2";

const INSERT_BUDGET_PLAN_CATEGORIES: &str = "INSERT INTO budget_plan_category (budget_category_id,budget_id) VALUES (?1,?2)";
const GET_ALL_BUDGET_PLAN_CATEGORIES: &str = "SELECT bc.id,bc.category_id,bc.flat_amount,bc.percentage_amount FROM budget_plan_category bpc JOIN budget_category bc ON bc.id = bpc.budget_category_id where bpc.budget_id = ?1";
const DELETE_BUDGET_PLAN_CATEGORY: &str = "DELETE FROM budget_plan_category WHERE budget_category_id = ?1 AND budget_id = ?2";

const GET_ALL_BUDGET_STATISTICS: &str = "SELECT
    c.name as category_name,
    bc.flat_amount as category_budget,
    ( SELECT sum(ti.amount) FROM transaction_item ti
        WHERE 
        ti.category_id = c.id
        AND ti.transaction_date BETWEEN b.start_date AND DATE(b.start_date, '+1 months')
    ) as category_spent
    FROM budget_budget_category bbc
    JOIN budget b
        ON b.id = bbc.budget_id
    JOIN budget_category bc
        ON bc.id = bbc.budget_category_id
    JOIN category c
        ON c.id = bc.category_id
    WHERE bbc.budget_id = ?1";

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

pub(crate) fn add_budget_category_sqlite(conn: &Connection,budget_category:BudgetCategory) -> anyhow::Result<()>{
    let result = insert_or_update_item(conn, (&budget_category.category_id,&budget_category.flat_amount,&budget_category.percentage_amount,&budget_category.fixed), INSERT_BUDGET_CATEGORY);

    Ok({})
}

pub(crate) fn update_budget_category_sqlite(conn: &Connection,budget_category: BudgetCategory) -> anyhow::Result<()>{
    let result = insert_or_update_item(conn,
         (&budget_category.id,&budget_category.category_id,&budget_category.flat_amount,&budget_category.percentage_amount,&budget_category.fixed),
        UPDATE_BUDGET_CATEGORY);
    
    Ok({})
}

pub(crate) fn remove_budget_category_sqlite(conn: &Connection,budget_category: BudgetCategory) -> anyhow::Result<()>{
    let result = remove_item(conn, DELETE_BUDGET_CATEGORY, budget_category.id);

    Ok({})
}

pub(crate) fn get_all_budget_categories_sqlite(conn: &Connection) -> anyhow::Result<Vec<BudgetCategory>> {
    let result = get_all::<BudgetCategory>(conn,GET_ALL_BUDGET_CATEGORIES);

    result
}

pub(crate) fn get_one_budget_category_sqlite(conn: &Connection,id: &str) -> anyhow::Result<BudgetCategory>{
    let result = get_one_by_id::<BudgetCategory>(conn, id, GET_ONE_BUDGET_CATEGORY);

    result
}

pub(crate) fn add_budget_sqlite(conn: &Connection,budget:Budget) -> anyhow::Result<()>{
    let result = insert_or_update_item(conn, (&budget.start_date,&budget.cycle), INSERT_BUDGET);

    Ok({})
}

pub(crate) fn update_budget_sqlite(conn: &Connection,budget:Budget) -> anyhow::Result<()>{
    let result = insert_or_update_item(conn,
         (&budget.id,&budget.start_date,&budget.cycle),
        UPDATE_BUDGET);
    
    Ok({})
}

pub(crate) fn remove_budget_sqlite(conn: &Connection,budget:Budget) -> anyhow::Result<()>{
    let result = remove_item(conn, DELETE_BUDGET, budget.id);

    Ok({})
}

pub(crate) fn get_all_budget_sqlite(conn: &Connection) -> anyhow::Result<Vec<Budget>> {
    let result = get_all::<Budget>(conn,GET_ALL_BUDGET);

    result
}

pub(crate) fn get_one_budget_sqlite(conn: &Connection,id: &str) -> anyhow::Result<Budget>{
    let result = get_one_by_id::<Budget>(conn, id, GET_ONE_BUDGET);

    result
}

pub(crate) fn add_budget_plan_sqlite(conn: &Connection,budget_plan:BudgetPlan) -> anyhow::Result<()>{
    let result = insert_or_update_item(conn, [&budget_plan.cycle], INSERT_BUDGET_PLAN);

    Ok({})
}

pub(crate) fn update_budget_plan_sqlite(conn: &Connection,budget_plan:BudgetPlan) -> anyhow::Result<()>{
    let result = insert_or_update_item(conn,
         (&budget_plan.id,&budget_plan.cycle),
        UPDATE_BUDGET_PLAN);
    
    Ok({})
}

pub(crate) fn remove_budget_plan_sqlite(conn: &Connection,budget_plan:BudgetPlan) -> anyhow::Result<()>{
    let result = remove_item(conn, DELETE_BUDGET_PLAN, budget_plan.id);

    Ok({})
}

pub(crate) fn get_all_budget_plan_sqlite(conn: &Connection) -> anyhow::Result<Vec<BudgetPlan>> {
    let result = get_all::<BudgetPlan>(conn,GET_ALL_BUDGET_PLAN);

    result
}

pub(crate) fn get_one_budget_plan_sqlite(conn: &Connection,id: &str) -> anyhow::Result<BudgetPlan>{
    let result = get_one_by_id::<BudgetPlan>(conn, id, GET_ONE_BUDGET_PLAN);

    result
}

pub(crate) fn add_budget_budget_category_sqlite(conn: &Connection,budget:Budget,budget_category:BudgetCategory) -> anyhow::Result<()>{
    let result = insert_or_update_item(conn, (&budget_category.id,&budget.id), INSERT_BUDGET_BUDGET_CATEGORIES);

    result
}

pub(crate) fn get_all_budget_budget_categories_sqlite(conn: &Connection, budget:Budget) -> anyhow::Result<Vec<BudgetCategory>>{
    let result = get_by_params(conn,[budget.id] ,GET_ALL_BUDGET_BUDGET_CATEGORIES);

    result
}

pub(crate) fn remove_budget_budget_category_sqlite(conn: &Connection,budget_budget_category:BudgetBudgetCategory) -> anyhow::Result<()>{
    let result = remove_item_params(conn,(&budget_budget_category.budget_category_id,&budget_budget_category.budget_id),DELETE_BUDGET_BUDGET_CATEGORY);

    result
}

pub(crate) fn add_budget_plan_category_sqlite(conn: &Connection,budget_plan:BudgetPlan,budget_category:BudgetCategory) -> anyhow::Result<()>{
    let result = insert_or_update_item(conn, (&budget_category.id,&budget_plan.id), INSERT_BUDGET_PLAN_CATEGORIES);

    result
}

pub(crate) fn get_all_budget_plan_categories_sqlite(conn: &Connection, budget_plan:BudgetPlan) -> anyhow::Result<Vec<BudgetCategory>>{
    let result = get_by_params(conn,[budget_plan.id] ,GET_ALL_BUDGET_PLAN_CATEGORIES);

    result
}

pub(crate) fn remove_budget_plan_category_sqlite(conn: &Connection,budget_plan_category:BudgetPlanCategory) -> anyhow::Result<()>{
    let result = remove_item_params(conn,(&budget_plan_category.budget_category_id,&budget_plan_category.budget_plan_id),DELETE_BUDGET_PLAN_CATEGORY);

    result
}

pub(crate) fn get_active_budget_statistics_sqlite(conn: &Connection,budget:Budget) -> anyhow::Result<Vec<BudgetStatisticsResponseModel>>{
    let result = get_by_params(conn, [budget.id], GET_ALL_BUDGET_STATISTICS);

    result
}


fn remove_item_params<P:Params>(conn: &Connection,params: P,command:&str) -> anyhow::Result<()>{
    let execute = conn.execute(command,
        params);
    if execute.is_ok() {
        Ok({})
    }
    else {
        Err(execute.unwrap_err().into())
    }
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

fn get_by_params<P:Params,T: for<'a> Deserialize<'a>>(conn: &Connection,params: P,command: &str) -> anyhow::Result<Vec<T>>{
    let prepared_stmt = conn.prepare(command);
    if prepared_stmt.is_err() {
        let error_msg = prepared_stmt.unwrap_err();
        println!("failed to prepare statement: {}",error_msg);
        return Err(error_msg.into());
    }
    let mut stmt = prepared_stmt.unwrap();
    let rows = stmt.query_map(params, |row| {
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