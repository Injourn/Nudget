import TransactionRequestModel from "../models/TransactionRequestModel";
import CategoryModel from "../models/CategoryModel";
import { useEffect, useState } from "react";
import GenericForm from "./forms/GenericForm";
import GenericFormInput from "./forms/GenericFormInput";
import GenericSelectInput from "./forms/GenericSelectInput";
import callTauri from "../functions/CallTauri";


function TransactionAddEdit(props: any){
    const [item,setItem] = useState<TransactionRequestModel>({} as TransactionRequestModel);
    const [categories,setCategories]= useState<CategoryModel[]>([]);

    useEffect(() => {
        setItem(props.entry);
    },[props.entry.id])

    useEffect(() => {   
        callTauri<CategoryModel[]>("get_all_categories").then(items => setCategories(items));
    },[]);


    function addTransaction(){
        let promise;
        if(item.id){
            promise = callTauri("update_transaction",{transaction: item})
        }
        else {
            promise = callTauri("add_transaction",{transaction: item})
        }
        promise.then(() => props.onSubmit());
        event?.preventDefault();
    }

    function removeItem(){
        callTauri("remove_transaction",{transaction: item}).then(() => props.onSubmit());
    }
    
    return(
        <GenericForm modalName={props.modalName} onSubmit={addTransaction} edit={item.id > 0} onRemove={removeItem}>
            <GenericFormInput onChange={(e) => setItem({...item, amount: e.target.value})} id={"amount"}
                label={"Amount"} item={item.amount} type={"text"} numeric={true}/>
            <GenericSelectInput onChange={(e) => setItem({...item, category_id: Number(e.target.value)})} id={"category"}
                label={"Category"} item={item.category_id}>

                {categories.map((data) =>
                    <option value={data.id}>{data.name}</option>
                )}
            </GenericSelectInput>
            <GenericFormInput 
             onChange={() =>{setItem({...item,recurring: !item.recurring})}}
             id={"recurring"} label={"Recurring"} 
             item={item.recurring} type={"checkbox"} />
            <GenericFormInput onChange={(e) => setItem({...item, transaction_date: e.target.value})} id={"date"}
                label={"Date"} item={item.transaction_date} type={"date"}/>
            <GenericFormInput onChange={(e) => setItem({...item, name: e.target.value})} id={"name"}
                label={"Name"} item={item.name} type={"text"}/>
        </GenericForm>
    )
}

export default TransactionAddEdit;