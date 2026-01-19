use std::sync::Mutex;

use log::{debug, info};
use rusqlite::Connection;
use tauri::State;

use crate::{functions::insert_recurring_transactions::insert_recurring_transactions, models::response::response::Response};

#[tauri::command]
pub(crate) fn load_file(
    conn_state: State<'_, Mutex<Connection>>,
     file_path: &str,
     handle: tauri::AppHandle) -> Response<()> {
    info!("Loading local file");
    debug!("Loading file name \"{}\"",file_path);
    let conn = Connection::open(file_path);
    let mut state = conn_state.inner().lock().unwrap();
    let response = match conn {
        Ok(_) => {
            *state = conn.unwrap();
            info!("Successfully loaded file");
            Response::success(())
        }
        Err(error) => Response::error(error.into()),
    };
    insert_recurring_transactions(&state,&handle);

    response
}
