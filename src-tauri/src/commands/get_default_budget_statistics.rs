use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::get_default_budget_statistics_sqlite,
    models::{
        request::transaction_in_range_request_model::TransactionInRangeRequestModel,
        response::{
            budget_statistics_response_model::BudgetStatisticsResponseModel, response::Response,
        },
    },
};

#[tauri::command]
pub(crate) fn get_default_budget_statistics(
    conn_state: State<'_, Mutex<Connection>>,
    range: TransactionInRangeRequestModel,
) -> Response<Vec<BudgetStatisticsResponseModel>> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();

    let result = get_default_budget_statistics_sqlite(conn, range);

    let response = match result {
        Ok(result) => Response::success(result),
        Err(error) => Response::error(error),
    };

    response
}
