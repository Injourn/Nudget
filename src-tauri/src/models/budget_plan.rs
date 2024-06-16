use super::cycle::Cycle;

#[derive(serde::Serialize,serde::Deserialize)]
pub(crate) struct BudgetPlan{
    pub(crate) id:u32,
    pub(crate) cycle: Cycle,
    pub(crate) name: String,
    pub(crate) active: bool,
    pub(crate) start_date_of_month:Option<u8>,
    pub(crate) start_date_of_week:Option<u8>
}