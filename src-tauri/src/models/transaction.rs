#[derive(serde::Serialize,serde::Deserialize)]
pub(crate)struct Transaction {
    pub(crate)id: u32,
    pub(crate)amount: String,
    pub(crate)category: String,
    pub(crate)date: String,
    pub(crate)name: String,
}

