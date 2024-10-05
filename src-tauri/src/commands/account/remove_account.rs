use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::remove_account_sqlite, models::{account::Account, response::response::Response}};


#[tauri::command]
pub(crate) fn remove_account(conn_state: State<'_, Mutex<Connection>>,account:Account) -> Response<()> {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    let result = remove_account_sqlite(conn, account);

    let response = match result {
        Ok(result) => Response::success(result),
        Err(error) => Response::error(error),
    };

    response
}
