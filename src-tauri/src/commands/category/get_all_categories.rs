use std::{ops::Deref, sync::Mutex};

use log::info;
use rusqlite::Connection;
use tauri::State;

use crate::{
    database::rusqlite_impl::get_all_categories_sqlite,
    models::{category::Category, response::response::Response},
};

#[tauri::command]
pub(crate) fn get_all_categories(
    conn_state: State<'_, Mutex<Connection>>,
) -> Response<Vec<Category>> {
    let conn = conn_state
        .inner()
        .lock()
        .expect("could not get db connection");
    let conn = conn.deref();
    info!("Getting all categories");

    let result = get_all_categories_sqlite(conn);

    let response = match result {
        Ok(result) => {
            info!("Successfully got all categories");    
            Response::success(result)
        },
        Err(error) => Response::error(error),
    };

    response
}
