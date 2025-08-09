use std::io::Read;

use tauri::{path::BaseDirectory, Manager};



pub(crate) fn load_sql_file(handle: tauri::AppHandle, file_location: &'static str) -> Result<String, anyhow::Error> {
    let resource_path = handle.path().resolve(file_location, BaseDirectory::Resource)?;
    let mut file = std::fs::File::open(&resource_path).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    Ok(contents)
}