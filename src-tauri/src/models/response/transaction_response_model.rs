#[derive(serde::Serialize,serde::Deserialize)]
pub(crate) struct TransactionResponseModel {
    pub(crate)id: u32,
    pub(crate)amount: String,
    pub(crate)category_id: u32,
    pub(crate)category_name: String,
    pub(crate)transaction_date: String,
    pub(crate)name: String,
}