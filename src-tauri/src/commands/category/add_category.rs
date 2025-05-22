use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::add_category_sqlite,
    models::{category::Category, response::response::Response},
};

#[tauri::command]
pub(crate) fn add_category(
    conn_state: State<'_, Mutex<Connection>>,
    category: Category,
) -> Response<i64> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    info!("Adding category");

    let result = add_category_sqlite(conn, category);

    let response = match result {
        Ok(result) => {
            info!("Successfully added category");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
