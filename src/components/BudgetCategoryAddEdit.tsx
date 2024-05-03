import { useState } from "react";
import BudgetCategoryModel from "../models/BudgetCategoryModel";
import CategoryModel from "../models/CategoryModel";
import { invoke } from "@tauri-apps/api";


function BudgetCategoryAddEdit(props:any){
    const item: BudgetCategoryModel = props.entry;
    const [categories,setCategories]= useState<CategoryModel[]>([]);
    invoke<CategoryModel[]>("get_all_categories").then(items => setCategories(items));
    
    function addBudgetCategory(formData:React.SyntheticEvent){
        item.fixed = false;
        console.log(item);
        invoke("add_budget_category",{budgetCategory: item});
        formData.preventDefault();
    }
    
    return(
        <form onSubmit={addBudgetCategory}>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <label htmlFor="amount" className="col-form-label">
                        Amount
                    </label>
                </div>
                <div className="col-auto">
                    <input type="text" id="amount" className="form-control" value={item.flat_amount} onChange={(e) => item.flat_amount = e.target.value}/>
                </div>
            </div>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <label htmlFor="catgory" className="col-form-label">
                        Category
                    </label>
                </div>
                <div className="col-auto">
                    <select name="category" id="category" className="form-select" value={item.category_id} onChange={(e) => item.category_id = Number(e.target.value)}>
                        {categories.map((data,i) =>
                            <option value={data.id}>{data.name}</option>
                        )}
                    </select>
                </div>
            </div>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <input type="submit" className="btn" data-bs-dismiss="modal" value={item.id ? "Edit Entry" : "Add Entry"} />
                </div>
            </div>
        </form>
    )

}

export default BudgetCategoryAddEdit;