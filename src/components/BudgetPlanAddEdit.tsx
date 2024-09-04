import { useState } from "react";
import GenericForm from "./forms/GenericForm";
import GenericSelectInput from "./forms/GenericSelectInput";
import BudgetPlanModel from "../models/BudgetPlanModel";
import Cycle from "../models/Cycle";
import GenericFormInput from "./forms/GenericFormInput";
import callTauri from "../functions/CallTauri";
import { useNavigate } from "react-router-dom";

interface BudgetPlanAddProps{
    entry?:BudgetPlanModel;
    onSubmit(): any;
}

function BudgetPlanAddEdit(props:BudgetPlanAddProps){
    const [item,setItem] = useState<BudgetPlanModel>(props.entry ?? {id:0, cycle: Cycle.MONTHLY} as BudgetPlanModel);
    const [showDayOfMonth,setShowDayOfMonth] = useState<boolean>(item.cycle == Cycle.MONTHLY);
    const navigate = useNavigate();

    function onSubmit(){
        if(item.cycle == Cycle.MONTHLY){
            item.start_date_of_week = undefined;
        } else {
            item.start_date_of_month = undefined;
        }
        if(item.id == 0){
            callTauri<number>("add_budget_plan",{budgetPlan:item}).then(id => navigate("/budgetPlan/" + id));
        } else {
            callTauri<number>("update_budget_plan",{budgetPlan:item})
        }
        props.onSubmit();
        event?.preventDefault();
    }

    function onRemove(){
        callTauri("remove_budget_plan",{budgetPlan:item})
        props.onSubmit();
    }


    return (
        <GenericForm onSubmit={onSubmit} edit={item.id > 0} onRemove={onRemove}>
            <GenericFormInput 
             onChange={(e) => item.name = e.target.value}
             id={"name"} 
             label={"Name"} 
             item={item.name} 
             type={"text"} />
            <GenericSelectInput 
             onChange={(e) => {setItem({...item,cycle: Cycle[e.target.value as keyof typeof Cycle]}); setShowDayOfMonth(item.cycle == Cycle.MONTHLY)}} 
             id={"cycle"} label={"Budget Cycle"} 
             item={item.cycle}>
                <option value={"MONTHLY"}>Monthly</option>
                <option value={"WEEKLY"}>Weekly</option>
                <option value={"BIWEEKLY"}>Biweekly</option>
            </GenericSelectInput>

            {showDayOfMonth ?
                <GenericFormInput onChange={(e) => setItem({...item,start_date_of_month: Number(e.target.value)})}
                 id={"startDayOfCycle"} label={"Start Day of the Month"} item={item.start_date_of_month} type={"text"} />
                
            :
                <GenericSelectInput onChange={(e) => setItem({...item,start_date_of_week: Number(e.target.value)})}
                 id={"startDayOfWeek"} label={"Start Day of the Week"} item={item.start_date_of_week}> 
                    <option value={0}>Sunday</option>
                    <option value={1}>Monday</option>
                    <option value={2}>Tuesday</option>
                    <option value={3}>Wednesday</option>
                    <option value={4}>Thursday</option>
                    <option value={5}>Friday</option>
                    <option value={6}>Saturday</option>

                </GenericSelectInput>
            }
            <GenericFormInput 
             onChange={() =>{setItem({...item,active: !item.active})}}
             id={"active"} label={"Active"} 
             item={item.active} type={"checkbox"} />
        </GenericForm>
    )

}

export default BudgetPlanAddEdit;