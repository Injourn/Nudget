#[derive(serde::Serialize)]
pub(crate)struct Listing {
    amount: String,
    category: String,
    date: String,
    name: String,
}

#[tauri::command]
pub(crate) fn listing() -> [Listing;3] {
    [Listing {
            amount:String::from("12.34"),
            category:String::from("Groceries"),
            date:String::from("01/01/2024"),
            name:String::from("WalMart")},
        Listing {
            amount:String::from("3.14"),
            category:String::from("Groceries"),
            date:String::from("01/01/2024"),
            name:String::from("WalMart")},
        Listing {
            amount:String::from("0.01"),
            category:String::from("Groceries"),
            date:String::from("01/01/2024"),
            name:String::from("WalMart")}]
}