use std::io::Read;

use tauri::{path::BaseDirectory, Manager};

const SQL_DIRECTORY:&str = "src/sql/";

pub(crate) fn load_sql_file(handle: tauri::AppHandle, file_name: &'static str) -> Result<String, anyhow::Error> {
    let resource_path = handle.path().resolve(SQL_DIRECTORY.to_owned() + file_name, BaseDirectory::Resource)?;
    let mut file = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    Ok(contents)
}