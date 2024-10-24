#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct BudgetBudgetCategory {
    pub(crate) budget_category_id: u32,
    pub(crate) budget_id: u32,
}
