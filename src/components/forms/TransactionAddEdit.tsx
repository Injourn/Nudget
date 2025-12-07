import TransactionRequestModel from "../../models/TransactionRequestModel";
import CategoryModel from "../../models/CategoryModel";
import { useEffect, useState } from "react";
import GenericForm from "./common/GenericForm";
import GenericFormInput from "./common/GenericFormInput";
import SelectInput from "./common/SelectInput";
import callTauri from "../../functions/CallTauri";
import Cycle from "../../models/Cycle";
import AccountInput from "./AccountInput";


function TransactionAddEdit(props: any){
    const [item,setItem] = useState<TransactionRequestModel>({} as TransactionRequestModel);
    const [categories,setCategories]= useState<CategoryModel[]>([]);
    const [showDayOfMonth,setShowDayOfMonth] = useState<boolean>(item.cycle == Cycle.MONTHLY);

    useEffect(() => {
        setItem(props.entry);
    },[props.entry.id,showDayOfMonth])

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
            <SelectInput onChange={(e) => setItem({...item, category_id: Number(e.target.value)})} id={"category"}
                label={"Category"} item={item.category_id}>

                {categories.map((data) =>
                    <option value={data.id}>{data.name}</option>
                )}
            </SelectInput>
            <GenericFormInput 
             onChange={() =>{setItem({...item,recurring: !item.recurring})}}
             id={"recurring"} label={"Recurring"} 
             item={item.recurring} type={"checkbox"} />
            {item.recurring ?
                <>
                    <SelectInput 
                        onChange={(e) => {
                            let cycle = e.target.value as keyof typeof Cycle;
                            setItem({...item,cycle: Cycle[cycle]});
                            setShowDayOfMonth(cycle == Cycle.MONTHLY)}} 
                        id={"cycle"} label={"Budget Cycle"} 
                        item={item.cycle}>
                        <option value={"MONTHLY"}>Monthly</option>
                        <option value={"WEEKLY"}>Weekly</option>
                        <option value={"BIWEEKLY"}>Biweekly</option>
                    </SelectInput>
                    {showDayOfMonth ?
                    <GenericFormInput onChange={(e) => setItem({...item,day_of_month: Number(e.target.value)})}
                    id={"startDayOfCycle"} label={"Start Day of the Month"} item={item.day_of_month} type={"text"} numeric={true} />
                    
                    :
                    <SelectInput onChange={(e) => setItem({...item,day_of_week: Number(e.target.value)})}
                    id={"startDayOfWeek"} label={"Start Day of the Week"} item={item.day_of_week}> 
                        <option value={0}>Sunday</option>
                        <option value={1}>Monday</option>
                        <option value={2}>Tuesday</option>
                        <option value={3}>Wednesday</option>
                        <option value={4}>Thursday</option>
                        <option value={5}>Friday</option>
                        <option value={6}>Saturday</option>

                    </SelectInput>
                    }
                </>
                :
                <GenericFormInput onChange={(e) => setItem({...item, transaction_date: e.target.value})} id={"date"}
                label={"Date"} item={item.transaction_date} type={"date"}/>
            }
            <GenericFormInput onChange={(e) => setItem({...item, name: e.target.value})} id={"name"}
                label={"Name"} item={item.name} type={"text"}/>
            
            <AccountInput item={item} setItem={setItem} ></AccountInput>
            <GenericFormInput 
             onChange={() =>{setItem({...item,credit: !item.credit})}}
             id={"credit"} label={"Credit"} 
             item={item.credit} type={"checkbox"} />
        </GenericForm>
    )
}

export default TransactionAddEdit;