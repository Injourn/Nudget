use std::{ops::Deref, sync::Mutex};

use log::{debug, info};
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::get_account_summary_in_range_sqlite,
    models::{
        request::account_summary_in_range_request::AccountSummaryInRangeRequest,
        response::{account_summary_response::AccountSummaryResponse, response::Response},
    },
};

#[tauri::command]
pub(crate) fn get_account_summary_in_range(
    conn_state: State<'_, Mutex<Connection>>,
    account_request: AccountSummaryInRangeRequest,
) -> Response<Option<AccountSummaryResponse>> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    info!("Getting account summary in range");
    debug!("Range: {} to {}", account_request.start_date, account_request.end_date);

    let result = get_account_summary_in_range_sqlite(conn, &account_request);

    let response = match result {
        Ok(result) => {
            info!("Successfully got account in range");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
