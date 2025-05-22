use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::update_account_sqlite,
    models::{account::Account, response::response::Response},
};

#[tauri::command]
pub(crate) fn update_account(
    conn_state: State<'_, Mutex<Connection>>,
    account: Account,
) -> Response<i64> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    info!("Updating account");
    let result = update_account_sqlite(conn, account);

    let response = match result {
        Ok(result) => {
            info!("Successfully updated account");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
