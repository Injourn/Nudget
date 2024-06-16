import { useState } from "react";
import BudgetCategoryModel from "../models/BudgetCategoryModel";
import CategoryModel from "../models/CategoryModel";
import { invoke } from "@tauri-apps/api";
import GenericForm from "./forms/GenericForm";
import GenericFormInput from "./forms/GenericFormInput";
import GenericSelectInput from "./forms/GenericSelectInput";


interface BudgetCategoryAddEditProps{
    parentAdd(newId: number): any;
    entry: BudgetCategoryModel;

}

function BudgetCategoryAddEdit(props:BudgetCategoryAddEditProps){
    const item: BudgetCategoryModel = props.entry;
    const [categories,setCategories]= useState<CategoryModel[]>([]);
    invoke<CategoryModel[]>("get_all_categories").then(items => setCategories(items));
    
    function addBudgetCategory(formData:React.SyntheticEvent){
        item.fixed = false;
        console.log(item);
        invoke<number>("add_budget_category",{budgetCategory: item}).then(newId => props.parentAdd(newId));
        formData.preventDefault();
    }
    
    return(
        <GenericForm onSubmit={addBudgetCategory}>
            <GenericFormInput onChange={(e) => item.flat_amount = e.target.value} id={"amount"}
             label={"Amount"} item={item.flat_amount} type={"text"} numeric={true}/>
            <GenericSelectInput onChange={(e) => item.category_id = Number(e.target.value)} id={"category"}
             label={"Category"} item={item.category_id}>
                {categories.map((data) =>
                    <option value={data.id}>{data.name}</option>
                )}
            </GenericSelectInput>
        </GenericForm>
    )

}

export default BudgetCategoryAddEdit;