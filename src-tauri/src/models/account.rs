#[derive(serde::Serialize,serde::Deserialize)]
pub(crate) struct Account {
    pub(crate) id:u32,
    pub(crate) name: String,
    pub(crate) created_date: String,
    pub(crate) currency_type: String
}