use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::remove_category_sqlite,
    models::{category::Category, response::response::Response},
};

#[tauri::command]
pub(crate) fn remove_category(
    conn_state: State<'_, Mutex<Connection>>,
    category: Category,
) -> Response<()> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    info!("Removing category by id");
    let result = remove_category_sqlite(conn, category);

    let response = match result {
        Ok(result) => {
            info!("Successfully removing category by id");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
