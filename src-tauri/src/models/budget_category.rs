#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct BudgetCategory {
    pub(crate) id: u32,
    pub(crate) category_id: u32,
    pub(crate) fixed: bool,
    pub(crate) flat_amount: String,
    pub(crate) percentage_amount: String,
}
