#[derive(serde::Serialize)]
pub(crate)struct Listing {
    pub(crate)amount: String,
    pub(crate)category: String,
    pub(crate)date: String,
    pub(crate)name: String,
}