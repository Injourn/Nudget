use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::get_active_budget_statistics_sqlite, models::{budget::Budget, response::{budget_statistics_response_model::BudgetStatisticsResponseModel, response::Response}}};


#[tauri::command]
pub(crate) fn get_active_budget_statistics(conn_state: State<'_, Mutex<Connection>>,budget:Budget) -> Response<Vec<BudgetStatisticsResponseModel>>{
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();

    let result = get_active_budget_statistics_sqlite(conn,budget);

    let response = match result {
        Ok(result) => Response::success(result),
        Err(error) => Response::error(error),
    };

    response
}