use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::add_account_sqlite,
    models::{account::Account, response::response::Response},
};

#[tauri::command]
pub(crate) fn add_account(
    conn_state: State<'_, Mutex<Connection>>,
    account: Account,
) -> Response<i64> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    info!("Adding account");

    let result = add_account_sqlite(conn, account);

    let response = match result {
        Ok(result) => {
            info!("Successfully added account");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
