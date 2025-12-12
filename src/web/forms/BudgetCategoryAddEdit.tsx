import { MouseEventHandler, useEffect, useState } from "react";
import BudgetCategoryModel from "../../models/entity/BudgetCategoryModel";
import callTauri from "../../functions/CallTauri";
import GenericForm from "./common/GenericForm";
import GenericFormInput from "./common/GenericFormInput";
import SelectInput from "./common/SelectInput";
import CategoryModel from "../../models/entity/CategoryModel";

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
        event?.preventDefault();
        item.fixed = false;
        console.debug(item);
        if(item.id > 0){
        callTauri<number>("update_budget_category",{budgetCategory: item})
        } else {
        callTauri<number>("add_budget_category",{budgetCategory: item}).then(newId => props.parentAdd(newId));
        }
        props.onSubmit();
    }

    function removeBudgetCategory() : MouseEventHandler<HTMLButtonElement> | undefined{
        callTauri("remove_budget_category",{budgetCategory: item});
        props.onSubmit();
        return undefined
    }

    function onChange(e:any){
        const value = e.target.value;
        const object : BudgetCategoryModel = item;
        const name = e.target.name as keyof typeof object;
        object[name] = value;

        setItem({...item,object});
    }
    
    return(
        <GenericForm onSubmit={addBudgetCategory} onRemove={removeBudgetCategory} edit={true}>
            <GenericFormInput onChange={onChange} id={"amount"}
             label={"Amount"} name={"flat_amount"} item={item.flat_amount} type={"text"} numeric={true}/>
            <SelectInput onChange={onChange} id={"category"}
             label={"Category"} name="category_id" item={item.category_id}>
                {categories.map((data) =>
                    <option value={data.id}>{data.name}</option>
                )}
            </SelectInput>
        </GenericForm>
    )

}

export default BudgetCategoryAddEdit;