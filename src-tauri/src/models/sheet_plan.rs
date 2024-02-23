use super::cycle::Cycle;

#[derive(serde::Serialize)]
pub(crate) struct SheetPlan{
    pub(crate) cycle: Cycle,
    pub(crate) start_day_of_month: i32,
    pub(crate) start_dat_of_week: i32,
}