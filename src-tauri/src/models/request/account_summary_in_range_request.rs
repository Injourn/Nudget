#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct AccountSummaryInRangeRequest {
    pub(crate) account_id: u32,
    pub(crate) start_date: String,
    pub(crate) end_date: String,
}
