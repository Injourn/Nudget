#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct AccountSummaryResponse {
    pub(crate) account_id: u32,
    pub(crate) name: String,
    pub(crate) credit_transactions: f64,
    pub(crate) debit_transactions: f64,
}
