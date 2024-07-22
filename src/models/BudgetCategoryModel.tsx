

interface BudgetCategoryModel{
    id:number,
    category_id: number,
    category_name?: string,
    fixed: boolean,
    flat_amount: string,
    percentage_amount: string,
}

export default BudgetCategoryModel;