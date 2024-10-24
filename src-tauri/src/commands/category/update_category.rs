use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::update_category_sqlite,
    models::{category, response::response::Response},
};

#[tauri::command]
pub(crate) fn update_category(
    conn_state: State<'_, Mutex<Connection>>,
    category: category::Category,
) -> Response<()> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    let result = update_category_sqlite(conn, category);

    let response = match result {
        Ok(_) => Response::success(()),
        Err(error) => Response::error(error),
    };

    response
}
