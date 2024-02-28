import CategoryModel from "./CategoryModel";
import SheetPlan from "./SheetPlanModel";


interface SheetCategoryModel{
    plan_parent: SheetPlan;
    category: CategoryModel;
    fixed: boolean;
    flat_amount: number;
    percentage_amount: number;
}

export default SheetCategoryModel;