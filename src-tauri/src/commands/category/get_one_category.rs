use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::get_one_category_sqlite,
    models::{category::Category, response::response::Response},
};

#[tauri::command]
pub(crate) fn get_one_category(
    conn_state: State<'_, Mutex<Connection>>,
    id: &str,
) -> Response<Option<Category>> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();

    let result = get_one_category_sqlite(conn, id);

    let response = match result {
        Ok(result) => Response::success(result),
        Err(error) => Response::error(error),
    };

    response
}
