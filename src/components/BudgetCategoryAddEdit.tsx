import { MouseEventHandler, useEffect, useState } from "react";
import BudgetCategoryModel from "../models/BudgetCategoryModel";
import CategoryModel from "../models/CategoryModel";
import callTauri from "../functions/CallTauri";
import GenericForm from "./forms/GenericForm";
import GenericFormInput from "./forms/GenericFormInput";
import GenericSelectInput from "./forms/GenericSelectInput";


interface BudgetCategoryAddEditProps{
    parentAdd(newId: number): any;
    entry: BudgetCategoryModel;
    onSubmit(): any;

}

function BudgetCategoryAddEdit(props:BudgetCategoryAddEditProps){
    const [item,setItem] = useState<BudgetCategoryModel>({} as BudgetCategoryModel);
    const [categories,setCategories]= useState<CategoryModel[]>([]);

    useEffect(() => {
        setItem(props.entry);
    },[props.entry]);
    
    useEffect(() => {
        callTauri<CategoryModel[]>("get_all_categories").then(items => setCategories(items));
    }, []);
    
    function addBudgetCategory(){
        item.fixed = false;
        console.log(item);
        callTauri<number>("add_budget_category",{budgetCategory: item}).then(newId => props.parentAdd(newId));
        props.onSubmit();
        event?.preventDefault();
    }

    function removeBudgetCategory() : MouseEventHandler<HTMLButtonElement> | undefined{
        callTauri("remove_budget_category",{budgetCategory: item});
        props.onSubmit();
        return undefined
    }
    
    return(
        <GenericForm onSubmit={addBudgetCategory} onRemove={removeBudgetCategory} edit={true}>
            <GenericFormInput onChange={(e) => setItem({...item,flat_amount: e.target.value})} id={"amount"}
             label={"Amount"} item={item.flat_amount} type={"text"} numeric={true}/>
            <GenericSelectInput onChange={(e) => setItem({...item,category_id: Number(e.target.value)})} id={"category"}
             label={"Category"} item={item.category_id}>
                {categories.map((data) =>
                    <option value={data.id}>{data.name}</option>
                )}
            </GenericSelectInput>
        </GenericForm>
    )

}

export default BudgetCategoryAddEdit;