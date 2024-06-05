use super::cycle::Cycle;

#[derive(serde::Serialize,serde::Deserialize)]
pub(crate) struct Budget{
    pub(crate) id:u32,
    pub(crate) start_date: String,
    pub(crate) cycle: Option<Cycle>,
    pub(crate) end_date: String,
}