use crate::models::cycle::Cycle;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct TransactionResponseModel {
    pub(crate) id: u32,
    pub(crate) amount: String,
    pub(crate) category_id: u32,
    pub(crate) category_name: String,
    pub(crate) transaction_date: String,
    pub(crate) name: String,
    pub(crate) recurring: bool,
    pub(crate) credit: bool,
    pub(crate) cycle: Option<Cycle>,
    pub(crate) day_of_month: Option<u8>,
    pub(crate) day_of_week: Option<u8>,
    pub(crate) account_id: Option<u32>,
    pub(crate) account_name: Option<String>,
}
