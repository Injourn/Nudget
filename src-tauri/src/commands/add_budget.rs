use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::{database::rusqlite_impl::add_budget_sqlite, models::budget::Budget};


#[tauri::command]
pub(crate) fn add_budget(conn_state: State<'_, Mutex<Connection>>,budget:Budget) {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();

    let result = add_budget_sqlite(conn,budget);
}
