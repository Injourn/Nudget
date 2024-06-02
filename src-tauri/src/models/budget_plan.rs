use super::cycle::Cycle;

#[derive(serde::Serialize,serde::Deserialize)]
pub(crate) struct BudgetPlan{
    pub(crate) id:u32,
    pub(crate) cycle: Cycle,
}