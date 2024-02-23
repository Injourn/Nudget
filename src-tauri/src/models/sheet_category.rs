use rust_decimal::prelude::Decimal;

use super::sheet_plan::SheetPlan;
use super::category::Category;

#[derive(serde::Serialize)]
pub(crate) struct SheetCategory{
    pub(crate) plan_parent: SheetPlan,
    pub(crate) category: Category,
    pub(crate) fixed: bool,
    pub(crate) flat_amount: Decimal,
    pub(crate) percentage_amount: Decimal,
    // probably not needed
    pub(crate) remaining: Decimal,
}