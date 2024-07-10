#[derive(serde::Serialize,serde::Deserialize)]
pub(crate) struct TransactionInRangeRequestModel{
    pub(crate) start_date:String,
    pub(crate) end_date:String
}