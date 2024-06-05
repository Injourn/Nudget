#[derive(serde::Serialize,serde::Deserialize)]
pub(crate) struct BudgetStatisticsResponseModel{
    pub(crate)category_name:String,
    pub(crate)category_budget:String,
    pub(crate)category_spent:f64,
}