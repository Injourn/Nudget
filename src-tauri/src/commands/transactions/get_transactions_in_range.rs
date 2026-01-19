use std::{cmp::Reverse, ops::Deref, sync::Mutex};

use chrono::NaiveDate;
use log::{debug, info};
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::get_transactions_in_range_sqlite,
    models::{
        request::transaction_in_range_request_model::TransactionInRangeRequestModel,
        response::{response::Response, transaction_response_model::TransactionResponseModel},
    },
};

use super::date_recurring_transactions::date_recurring_transactions;

#[tauri::command]
pub(crate) fn get_transactions_in_range(
    conn_state: State<'_, Mutex<Connection>>,
    transaction_request: TransactionInRangeRequestModel,
) -> Response<Vec<TransactionResponseModel>> {
    info!("Running Command: Gettings all transctions in a range.");
    debug!("Range: {} to {}", &transaction_request.start_date, &transaction_request.end_date);
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();

    let result = get_transactions_in_range_sqlite(conn, &transaction_request);

    let response = match result {
        Ok(mut result) => {
            // TODO: replace with upcoming recurring transaction
            // let result_mut = result.as_mut();
            // date_recurring_transactions(
            //     result_mut,
            //     &transaction_request.start_date,
            //     &transaction_request.end_date,
            // );
            result.sort_by_key(|a| {
                Reverse(
                    NaiveDate::parse_from_str(&a.transaction_date, "%Y-%m-%d")
                        .expect("failed to parse date"),
                )
            });

            info!("Successfully got all transactions in range");
            Response::success(result)
        }
        Err(error) => Response::error(error),
    };

    response
}
