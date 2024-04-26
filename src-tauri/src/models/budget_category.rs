use rust_decimal::prelude::Decimal;


#[derive(serde::Serialize)]
pub(crate) struct BudgetCategory{
    pub(crate) category_id: u32,
    pub(crate) fixed: bool,
    pub(crate) flat_amount: Decimal,
    pub(crate) percentage_amount: Decimal,
    // probably not needed
    pub(crate) remaining: Decimal,
}