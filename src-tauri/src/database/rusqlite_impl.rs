use rusqlite::{Connection, Params};
use serde::Deserialize;

use crate::models::{
    budget::Budget,
    budget_budget_category::BudgetBudgetCategory,
    budget_category::BudgetCategory,
    budget_plan::BudgetPlan,
    budget_plan_category::BudgetPlanCategory,
    category::Category,
    request::transaction_in_range_request_model::TransactionInRangeRequestModel,
    response::{
        budget_category_response_model::BudgetCategoryResponse, budget_statistics_response_model::BudgetStatisticsResponseModel, transaction_response_model::TransactionResponseModel
    },
    transaction::Transaction,
};

use super::sql_constants::{
    ADD_TRANSACTION, DELETE_BUDGET, DELETE_BUDGET_BUDGET_CATEGORY, DELETE_BUDGET_CATEGORY, DELETE_BUDGET_PLAN, DELETE_BUDGET_PLAN_CATEGORY, DELETE_CATEGORY, DELETE_TRANSACTION, GET_ACTIVE_BUDGET_PLAN, GET_ALL_BUDGET, GET_ALL_BUDGET_BUDGET_CATEGORIES, GET_ALL_BUDGET_CATEGORIES, GET_ALL_BUDGET_PLAN, GET_ALL_BUDGET_PLAN_CATEGORIES, GET_ALL_BUDGET_STATISTICS, GET_ALL_CATEGORIES, GET_ALL_DEFAULT_BUDGET_STATISTICS, GET_ALL_TRANSACTIONS, GET_ALL_TRANSACTIONS_IN_RANGE, GET_ONE_BUDGET, GET_ONE_BUDGET_BY_DATE, GET_ONE_BUDGET_CATEGORY, GET_ONE_BUDGET_PLAN, GET_ONE_CATEGORY, GET_ONE_TRANSACTION, INSERT_BUDGET, INSERT_BUDGET_BUDGET_CATEGORIES, INSERT_BUDGET_CATEGORY, INSERT_BUDGET_PLAN, INSERT_BUDGET_PLAN_CATEGORIES, INSERT_CATEGORY, UPDATE_BUDGET, UPDATE_BUDGET_CATEGORY, UPDATE_BUDGET_PLAN, UPDATE_CATEGORY, UPDATE_TRANSACTION
};

pub(crate) fn get_transaction_sqlite(
    conn: &Connection,
) -> anyhow::Result<Vec<TransactionResponseModel>> {
    let result = get_all::<TransactionResponseModel>(conn, GET_ALL_TRANSACTIONS);

    result
}

pub(crate) fn get_one_transaction_sqlite(
    conn: &Connection,
    id: &str,
) -> anyhow::Result<TransactionResponseModel> {
    let result = get_one_by_id::<TransactionResponseModel>(conn, id, GET_ONE_TRANSACTION);

    result
}
pub(crate) fn add_transaction_sqlite(
    conn: &Connection,
    transaction: Transaction,
) -> anyhow::Result<i64> {
    let result = insert_or_update_item(
        conn,
        (
            &transaction.amount,
            &transaction.category_id,
            &transaction.transaction_date,
            &transaction.name,
            &transaction.recurring
        ),
        ADD_TRANSACTION,
    );

    result
}

pub(crate) fn update_transaction_sqlite(
    conn: &Connection,
    transaction: Transaction,
) -> anyhow::Result<i64> {
    let result = insert_or_update_item(
        conn,
        (
            &transaction.id,
            &transaction.amount,
            &transaction.category_id,
            &transaction.transaction_date,
            &transaction.name,
            &transaction.recurring
        ),
        UPDATE_TRANSACTION,
    );

    result
}

pub(crate) fn remove_transaction_sqlite(
    conn: &Connection,
    transaction: Transaction,
) -> anyhow::Result<()> {
    let result = remove_item(conn, DELETE_TRANSACTION, transaction.id);

    result
}

pub(crate) fn add_category_sqlite(conn: &Connection, category: Category) -> anyhow::Result<i64> {
    let result = insert_or_update_item(conn, [category.name], INSERT_CATEGORY);

    result
}

pub(crate) fn update_category_sqlite(conn: &Connection, category: Category) -> anyhow::Result<i64> {
    let result = insert_or_update_item(conn, (&category.id, &category.name), UPDATE_CATEGORY);

    result
}

pub(crate) fn remove_category_sqlite(conn: &Connection, category: Category) -> anyhow::Result<()> {
    let result = remove_item(conn, DELETE_CATEGORY, category.id);

    result
}

pub(crate) fn get_all_categories_sqlite(conn: &Connection) -> anyhow::Result<Vec<Category>> {
    let result = get_all::<Category>(conn, GET_ALL_CATEGORIES);

    result
}

pub(crate) fn get_one_category_sqlite(conn: &Connection, id: &str) -> anyhow::Result<Category> {
    let result = get_one_by_id::<Category>(conn, id, GET_ONE_CATEGORY);

    result
}

pub(crate) fn add_budget_category_sqlite(
    conn: &Connection,
    budget_category: BudgetCategory,
) -> anyhow::Result<i64> {
    let result = insert_or_update_item(
        conn,
        (
            &budget_category.category_id,
            &budget_category.flat_amount,
            &budget_category.percentage_amount,
            &budget_category.fixed,
        ),
        INSERT_BUDGET_CATEGORY,
    );

    result
}

pub(crate) fn update_budget_category_sqlite(
    conn: &Connection,
    budget_category: BudgetCategory,
) -> anyhow::Result<i64> {
    let result = insert_or_update_item(
        conn,
        (
            &budget_category.id,
            &budget_category.category_id,
            &budget_category.flat_amount,
            &budget_category.percentage_amount,
            &budget_category.fixed,
        ),
        UPDATE_BUDGET_CATEGORY,
    );

    result
}

pub(crate) fn remove_budget_category_sqlite(
    conn: &Connection,
    budget_category: BudgetCategory,
) -> anyhow::Result<()> {
    let result = remove_item(conn, DELETE_BUDGET_CATEGORY, budget_category.id);

    result
}

pub(crate) fn get_all_budget_categories_sqlite(
    conn: &Connection,
) -> anyhow::Result<Vec<BudgetCategory>> {
    let result = get_all::<BudgetCategory>(conn, GET_ALL_BUDGET_CATEGORIES);

    result
}

pub(crate) fn get_one_budget_category_sqlite(
    conn: &Connection,
    id: &str,
) -> anyhow::Result<BudgetCategory> {
    let result = get_one_by_id::<BudgetCategory>(conn, id, GET_ONE_BUDGET_CATEGORY);

    result
}

pub(crate) fn get_one_budget_by_date_sqlite(
    conn: &Connection,
    range: &str,
) -> anyhow::Result<Vec<Budget>> {
    let result = get_by_params(conn, [range], GET_ONE_BUDGET_BY_DATE);

    result
}

pub(crate) fn add_budget_sqlite(conn: &Connection, budget: Budget) -> anyhow::Result<i64> {
    let result = insert_or_update_item(
        conn,
        (&budget.start_date, &budget.cycle, &budget.cycle),
        INSERT_BUDGET,
    );

    result
}

pub(crate) fn update_budget_sqlite(conn: &Connection, budget: Budget) -> anyhow::Result<i64> {
    let result = insert_or_update_item(
        conn,
        (
            &budget.id,
            &budget.start_date,
            &budget.cycle,
            &budget.end_date,
        ),
        UPDATE_BUDGET,
    );

    result
}

pub(crate) fn remove_budget_sqlite(conn: &Connection, budget: Budget) -> anyhow::Result<()> {
    let result = remove_item(conn, DELETE_BUDGET, budget.id);

    result
}

pub(crate) fn get_all_budget_sqlite(conn: &Connection) -> anyhow::Result<Vec<Budget>> {
    let result = get_all::<Budget>(conn, GET_ALL_BUDGET);

    result
}

pub(crate) fn get_one_budget_sqlite(conn: &Connection, id: &str) -> anyhow::Result<Budget> {
    let result = get_one_by_id::<Budget>(conn, id, GET_ONE_BUDGET);

    result
}

pub(crate) fn add_budget_plan_sqlite(
    conn: &Connection,
    budget_plan: BudgetPlan,
) -> anyhow::Result<i64> {
    let result = insert_or_update_item(
        conn,
        (
            &budget_plan.cycle,
            &budget_plan.start_date_of_month,
            &budget_plan.start_date_of_week,
            &budget_plan.name,
            &budget_plan.active,
        ),
        INSERT_BUDGET_PLAN,
    );

    result
}

pub(crate) fn update_budget_plan_sqlite(
    conn: &Connection,
    budget_plan: BudgetPlan,
) -> anyhow::Result<i64> {
    let result = insert_or_update_item(
        conn,
        (
            &budget_plan.id,
            &budget_plan.cycle,
            &budget_plan.start_date_of_month,
            &budget_plan.start_date_of_week,
            &budget_plan.name,
            &budget_plan.active,
        ),
        UPDATE_BUDGET_PLAN,
    );

    result
}

pub(crate) fn remove_budget_plan_sqlite(
    conn: &Connection,
    budget_plan: BudgetPlan,
) -> anyhow::Result<()> {
    let result = remove_item(conn, DELETE_BUDGET_PLAN, budget_plan.id);

    result
}

pub(crate) fn get_all_budget_plan_sqlite(conn: &Connection) -> anyhow::Result<Vec<BudgetPlan>> {
    let result = get_all::<BudgetPlan>(conn, GET_ALL_BUDGET_PLAN);

    result
}

pub(crate) fn get_one_budget_plan_sqlite(
    conn: &Connection,
    id: &str,
) -> anyhow::Result<BudgetPlan> {
    let result = get_one_by_id::<BudgetPlan>(conn, id, GET_ONE_BUDGET_PLAN);

    result
}

pub(crate) fn get_active_budget_plan_sqlite(
    conn: &Connection,
) -> anyhow::Result<Option<BudgetPlan>> {
    let result = get_one::<BudgetPlan>(conn, GET_ACTIVE_BUDGET_PLAN);

    result
}

pub(crate) fn add_budget_budget_category_sqlite(
    conn: &Connection,
    budget_id: u32,
    budget_category_id: u32,
) -> anyhow::Result<i64> {
    let result = insert_or_update_item(
        conn,
        (budget_category_id, budget_id),
        INSERT_BUDGET_BUDGET_CATEGORIES,
    );

    result
}

pub(crate) fn get_all_budget_budget_categories_sqlite(
    conn: &Connection,
    budget: Budget,
) -> anyhow::Result<Vec<BudgetCategoryResponse>> {
    let result = get_by_params(conn, [budget.id], GET_ALL_BUDGET_BUDGET_CATEGORIES);

    result
}

pub(crate) fn remove_budget_budget_category_sqlite(
    conn: &Connection,
    budget_budget_category: BudgetBudgetCategory,
) -> anyhow::Result<()> {
    let result = remove_item_params(
        conn,
        (
            &budget_budget_category.budget_category_id,
            &budget_budget_category.budget_id,
        ),
        DELETE_BUDGET_BUDGET_CATEGORY,
    );

    result
}

pub(crate) fn add_budget_plan_category_sqlite(
    conn: &Connection,
    budget_plan_id: u32,
    budget_category_id: u32,
) -> anyhow::Result<i64> {
    let result = insert_or_update_item(
        conn,
        (&budget_category_id, &budget_plan_id),
        INSERT_BUDGET_PLAN_CATEGORIES,
    );

    result
}

pub(crate) fn get_all_budget_plan_categories_sqlite(
    conn: &Connection,
    budget_plan: BudgetPlan,
) -> anyhow::Result<Vec<BudgetCategoryResponse>> {
    let result = get_by_params(conn, [budget_plan.id], GET_ALL_BUDGET_PLAN_CATEGORIES);

    result
}

pub(crate) fn remove_budget_plan_category_sqlite(
    conn: &Connection,
    budget_plan_category: BudgetPlanCategory,
) -> anyhow::Result<()> {
    let result = remove_item_params(
        conn,
        (
            &budget_plan_category.budget_category_id,
            &budget_plan_category.budget_plan_id,
        ),
        DELETE_BUDGET_PLAN_CATEGORY,
    );

    result
}

pub(crate) fn get_active_budget_statistics_sqlite(
    conn: &Connection,
    budget: Budget,
) -> anyhow::Result<Vec<BudgetStatisticsResponseModel>> {
    let result = get_by_params(conn, [budget.id], GET_ALL_BUDGET_STATISTICS);

    result
}
pub(crate) fn get_default_budget_statistics_sqlite(
    conn: &Connection,
    range: TransactionInRangeRequestModel,
) -> anyhow::Result<Vec<BudgetStatisticsResponseModel>> {
    let result = get_by_params(conn, (&range.start_date,&range.end_date), GET_ALL_DEFAULT_BUDGET_STATISTICS);

    result
}

pub(crate) fn get_transactions_in_range_sqlite(
    conn: &Connection,
    request: &TransactionInRangeRequestModel,
) -> anyhow::Result<Vec<TransactionResponseModel>> {
    let result = get_by_params(
        conn,
        (&request.start_date, &request.end_date),
        GET_ALL_TRANSACTIONS_IN_RANGE,
    );

    result
}

fn remove_item_params<P: Params>(
    conn: &Connection,
    params: P,
    command: &str,
) -> anyhow::Result<()> {
    let execute = conn.execute(command, params);
    if execute.is_ok() {
        Ok({})
    } else {
        let error = execute.unwrap_err();
        println!("Error: {}", error);
        Err(error.into())
    }
}

fn remove_item(conn: &Connection, command: &str, id: u32) -> anyhow::Result<()> {
    let execute = conn.execute(command, [id]);
    if execute.is_ok() {
        Ok({})
    } else {
        let error = execute.unwrap_err();
        println!("Error: {}", error);
        Err(error.into())
    }
}

fn insert_or_update_item<P: Params>(
    conn: &Connection,
    params: P,
    command: &str,
) -> anyhow::Result<i64> {
    let execute = conn.execute(command, params);
    if execute.is_ok() {
        Ok(conn.last_insert_rowid())
    } else {
        let error = execute.unwrap_err();
        println!("Error: {}", error);
        Err(error.into())
    }
}

fn get_one<T: for<'a> Deserialize<'a>>(
    conn: &Connection,
    command: &str,
) -> anyhow::Result<Option<T>> {
    let prepared_stmt = conn.prepare(command);
    if prepared_stmt.is_err() {
        let error_msg = prepared_stmt.unwrap_err();
        println!("failed to prepare statement: {}", error_msg);
        return Err(error_msg.into());
    }
    let mut stmt = prepared_stmt.unwrap();
    let mut rows = stmt.query_map([],map_rows::<T>)?;

    let transaction: Option<T> = match rows.nth(0){
        Some(item) => Some(item.unwrap()),
        None => None
    };
    rows.last();

    stmt.finalize().unwrap();

    Ok(transaction)
}

fn get_one_by_id<T: for<'a> Deserialize<'a>>(
    conn: &Connection,
    id: &str,
    command: &str,
) -> anyhow::Result<T> {
    let parsed_id: u32 = id.parse().unwrap();
    let prepared_stmt = conn.prepare(command);
    if prepared_stmt.is_err() {
        let error_msg = prepared_stmt.unwrap_err();
        println!("failed to prepare statement: {}", error_msg);
        return Err(error_msg.into());
    }
    let mut stmt = prepared_stmt.unwrap();
    let mut rows = stmt.query_map([parsed_id],map_rows)?;

    let transaction: T = rows.nth(0).unwrap()?;
    rows.last();

    stmt.finalize().unwrap();

    Ok(transaction)
}

fn get_all<T: for<'a> Deserialize<'a>>(conn: &Connection, command: &str) -> anyhow::Result<Vec<T>> {
    let prepared_stmt = conn.prepare(command);
    if prepared_stmt.is_err() {
        let error_msg = prepared_stmt.unwrap_err();
        println!("failed to prepare statement: {}", error_msg);
        return Err(error_msg.into());
    }
    let mut stmt = prepared_stmt.unwrap();
    let rows = stmt.query_map([], map_rows)?;

    let mut transactions: Vec<T> = Vec::new();

    for transaction in rows {
        transactions.push(transaction?);
    }

    stmt.finalize().unwrap();

    Ok(transactions)
}

fn get_by_params<P: Params, T: for<'a> Deserialize<'a>>(
    conn: &Connection,
    params: P,
    command: &str,
) -> anyhow::Result<Vec<T>> {
    let prepared_stmt = conn.prepare(command);
    if prepared_stmt.is_err() {
        let error_msg = prepared_stmt.unwrap_err();
        println!("failed to prepare statement: {}", error_msg);
        return Err(error_msg.into());
    }
    let mut stmt = prepared_stmt.unwrap();
    let rows = stmt.query_map(params,map_rows)?;

    let mut transactions: Vec<T> = Vec::new();

    for transaction in rows {
        transactions.push(transaction?);
    }

    stmt.finalize().unwrap();

    Ok(transactions)
}

fn map_rows<T: for<'a> Deserialize<'a>>(row: &rusqlite::Row) -> Result<T, rusqlite::Error> {
    let mapped_row_result = serde_rusqlite::from_row::<T>(row);
    let mapped_row = match mapped_row_result {
        Ok(result) => result,
        Err(error) => panic!("Invalid row name \"{row:?}\" ({error:?}"),
    };
    Ok(mapped_row)
}
