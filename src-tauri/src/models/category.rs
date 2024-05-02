#[derive(serde::Serialize,serde::Deserialize)]
pub(crate)struct Category{
    pub(crate) id:u32,
    pub(crate) name: String,
}