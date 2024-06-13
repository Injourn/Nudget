import CategoryModel from "./CategoryModel";
import BudgetPlanModel from "./BudgetPlanModel";


interface SheetCategoryModel{
    plan_parent: BudgetPlanModel;
    category: CategoryModel;
    fixed: boolean;
    flat_amount: number;
    percentage_amount: number;
}

export default SheetCategoryModel;