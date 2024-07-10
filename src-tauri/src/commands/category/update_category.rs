use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::update_category_sqlite, models::category};


#[tauri::command]
pub(crate) fn update_category(conn_state: State<'_, Mutex<Connection>>,category:category::Category) {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    update_category_sqlite(conn, category);
}
