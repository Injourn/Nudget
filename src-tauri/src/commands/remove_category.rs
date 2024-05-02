use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::remove_category_sqlite, models::category::Category};


#[tauri::command]
pub(crate) fn remove_category(conn_state: State<'_, Mutex<Connection>>,category:Category) {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    remove_category_sqlite(conn, category);
}
