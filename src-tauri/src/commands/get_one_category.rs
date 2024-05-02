use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::get_one_category_sqlite, models::category::Category};


#[tauri::command]
pub(crate) fn get_one_category(conn_state: State<'_, Mutex<Connection>>,id: &str) -> Category{
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();

    let result = get_one_category_sqlite(conn, id).expect("failed to fetch");

    result
}
