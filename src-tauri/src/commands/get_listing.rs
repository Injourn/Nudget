use crate::models::listing::Listing;

#[tauri::command]
pub(crate) fn get_listing() -> [Listing;3] {
    [Listing {
            amount:String::from("12.34"),
            category:String::from("Groceries"),
            date:String::from("2024-01-31"),
            name:String::from("WalMart")},
        Listing {
            amount:String::from("3.14"),
            category:String::from("Groceries"),
            date:String::from("2024-01-31"),
            name:String::from("WalMart")},
        Listing {
            amount:String::from("0.01"),
            category:String::from("Groceries"),
            date:String::from("2024-01-31"),
            name:String::from("WalMart")}]
}