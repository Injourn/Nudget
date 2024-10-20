use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::models::response::response::Response;

#[tauri::command]
pub(crate) fn load_file(conn_state: State<'_, Mutex<Connection>>, file_path: &str) -> Response<()> {
    let conn = Connection::open(file_path);
    let mut state = conn_state.inner().lock().unwrap();
    let response = match conn {
        Ok(_) => {
            *state = conn.unwrap();
            Response::success(())
        }
        Err(error) => Response::error(error.into()),
    };

    response
}
