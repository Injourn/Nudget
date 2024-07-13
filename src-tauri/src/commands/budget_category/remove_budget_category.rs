use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::remove_budget_category_sqlite, models::{budget_category::BudgetCategory, response::response::Response}};


#[tauri::command]
pub(crate) fn remove_budget_category(conn_state: State<'_, Mutex<Connection>>,budget_category:BudgetCategory) -> Response<()> {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    let result = remove_budget_category_sqlite(conn, budget_category);

    let response = match result {
        Ok(result) => Response::success(result),
        Err(error) => Response::error(error),
    };

    response
}
