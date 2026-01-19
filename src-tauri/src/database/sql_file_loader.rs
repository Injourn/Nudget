use std::io::Read;

use tauri::{path::BaseDirectory, Manager};

const SQL_DIRECTORY:&str = "src/sql/";

pub(crate) fn load_sql_file(handle: &tauri::AppHandle, file_name: &'static str) -> Result<String, std::io::Error> {
    let resource_path = handle.path().resolve(SQL_DIRECTORY.to_owned() + file_name, BaseDirectory::Resource).unwrap();
    let file = std::fs::File::open(&resource_path);
    let result = match file {
        Ok(mut f) => {
            let mut contents = String::new();
            let _ = f.read_to_string(&mut contents);
            Ok(contents)
        },
        Err(e) => Err(e),  
    };
    result
    
}