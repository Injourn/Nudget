use super::cycle::Cycle;

#[derive(serde::Serialize)]
pub(crate) struct BudgetPlan{
    pub(crate) cycle: Cycle,
}