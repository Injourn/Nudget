use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::get_one_budget_by_date_sqlite, models::{budget::Budget, response::response::Response}};


#[tauri::command]
pub(crate) fn get_one_budget_by_date(conn_state: State<'_, Mutex<Connection>>,range: &str) -> Response<Option<Budget>>{
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();

    let result = get_one_budget_by_date_sqlite(conn, range);

    let response = match result {
        Ok(result) => Response::success(result.first().cloned()),
        Err(error) => Response::error(error),
    };

    response
}