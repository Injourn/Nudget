use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::get_one_account_sqlite,
    models::{account::Account, response::response::Response},
};

#[tauri::command]
pub(crate) fn get_one_account(
    conn_state: State<'_, Mutex<Connection>>,
    id: &str,
) -> Response<Option<Account>> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    info!("Getting account by id");

    let result = get_one_account_sqlite(conn, id);

    let response = match result {
        Ok(result) => {
            info!("Successfully got account by id");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
