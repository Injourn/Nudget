use std::{ops::Deref, sync::Mutex};

use log::info;
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
    info!("Getting category by id");

    let result = get_one_category_sqlite(conn, id);

    let response = match result {
        Ok(result) => {
            info!("Successfully got all categories by id");
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
