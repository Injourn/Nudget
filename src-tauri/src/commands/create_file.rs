use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::{database::sql_constants::SQL_BUILD, models::response::response::Response};

#[tauri::command]
pub(crate) fn create_file(
    conn_state: State<'_, Mutex<Connection>>,
    file_path: &str,
) -> Response<()> {
    let mut file_path_owned = file_path.to_owned();
    if !file_path_owned.ends_with(".db") {
        file_path_owned = file_path_owned + ".db";
    }
    let conn = Connection::open(file_path_owned);
    let mut state = conn_state.inner().lock().unwrap();
    let response = match conn {
        Ok(conn) => {
            let _ = conn.execute_batch(SQL_BUILD);
            *state = conn;
            Response::success(())
        }
        Err(error) => Response::error(error.into()),
    };

    response
}
