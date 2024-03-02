use std::{ops::Deref, sync::Mutex};

use rusqlite::Connection;
use tauri::State;

use crate::models::listing::Listing;

#[tauri::command]
pub(crate) fn get_listing(conn_state: State<'_, Mutex<Connection>>) -> Vec<Listing> {
    let conn = conn_state.inner().lock().expect("could not get db connection");
    let conn = conn.deref();
    let result = get_listing_sqlite(conn).expect("failed to fetch");

    result
}

fn get_listing_sqlite(conn: &Connection) -> anyhow::Result<Vec<Listing>> {
    let mut stmt = conn.prepare("Select id, amount, category, date, name FROM listing").expect("fail");
    let rows = stmt.query_map([], |row| {
        Ok(serde_rusqlite::from_row::<Listing>(row)
            .expect("failed here"))
    })?;
    
    let mut listings: Vec<Listing> = Vec::new();

    for listing in rows {
        listings.push(listing?);
    }

    stmt.finalize().unwrap();
    
    Ok(listings)
}