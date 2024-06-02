#[derive(serde::Serialize,serde::Deserialize)]
pub(crate) struct BudgetPlanCategory{
    pub(crate)budget_category_id: u32,
    pub(crate)budget_plan_id: u32,
}