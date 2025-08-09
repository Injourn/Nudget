use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::{State};

use crate::{database::{rusqlite_impl::get_one_by_id, sql_file_loader::load_sql_file}, models::response::response::Response};
const SQL_FILE_PATH: &str = "src/sql/get_account_net_value.sql";


#[tauri::command]
pub(crate) fn get_account_net_value(
    conn_state: State<'_, Mutex<Connection>>,
    id: &str,
    handle: tauri::AppHandle
) -> Response<Option<f64>> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    info!("Getting account value");

    let result = get_result(conn,id, handle);

    let response = match result {
        Ok(result) => {
            info!("Successfully got account net value");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response

}

fn get_result(
    conn: &Connection,
    id: &str,
    handle: tauri::AppHandle
) -> anyhow::Result<Option<f64>> {
    let contents = load_sql_file(handle, SQL_FILE_PATH)?;
    get_one_by_id(conn,id, &contents)
}