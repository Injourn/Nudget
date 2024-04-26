#[derive(serde::Serialize)]
pub(crate) struct Budget{
    pub(crate) start_date: DateTime,
    pub(crate) cycle: Cycle,
}