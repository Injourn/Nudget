import TransactionRequestModel from "../models/TransactionRequestModel";
import CategoryModel from "../models/CategoryModel";
import { useState } from "react";
import GenericForm from "./forms/GenericForm";
import GenericFormInput from "./forms/GenericFormInput";
import GenericSelectInput from "./forms/GenericSelectInput";
import callTauri from "../functions/CallTauri";


function TransactionAddEdit(props: any){
    const item: TransactionRequestModel = props.entry;
    const [categories,setCategories]= useState<CategoryModel[]>([])
    callTauri<CategoryModel[]>("get_all_categories").then(items => setCategories(items));

    function addTransaction(formData:React.SyntheticEvent){
        console.log(item);
        if(item.id){
            callTauri("update_transaction",{transaction: item})
        }
        else {
            callTauri("add_transaction",{transaction: item})
        }
        formData.preventDefault();
    }

    function removeItem(){
        callTauri("remove_transaction",{transaction: item})
    }
    
    return(
        <GenericForm modalName={props.modalName} onSubmit={addTransaction} edit={item.id > 0} onRemove={removeItem}>
            <GenericFormInput onChange={(e) => item.amount = e.target.value} id={"amount"}
                label={"Amount"} item={item.amount} type={"text"} numeric={true}/>
            <GenericSelectInput onChange={(e) => item.category_id = Number(e.target.value)} id={"category"}
                label={"Category"} item={item.category_id}>

                {categories.map((data) =>
                    <option value={data.id}>{data.name}</option>
                )}
            </GenericSelectInput>
            <GenericFormInput onChange={(e) => item.transaction_date = e.target.value} id={"date"}
                label={"Date"} item={item.transaction_date} type={"date"}/>
            <GenericFormInput onChange={(e) => item.name = e.target.value} id={"name"}
                label={"Name"} item={item.name} type={"text"}/>
        </GenericForm>
    )
}

export default TransactionAddEdit;