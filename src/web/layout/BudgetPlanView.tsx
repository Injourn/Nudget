import { useEffect, useState } from "react";
import BudgetPlanCategoryTable from "./BudgetPlanCategoryTable"
import { useParams } from "react-router-dom";
import callTauri from "../../functions/CallTauri";
import BudgetPlanAddEdit from "../forms/BudgetPlanAddEdit";
import BudgetPlanModel from "../../models/entity/BudgetPlanModel";


function BudgetPlanView(props:any){
    let params = useParams();
    const [budgetPlan,setBudgetPlan] = useState<BudgetPlanModel>({} as BudgetPlanModel);
    let budgetPlanId = params.budgetPlanId ?? props.budgetPlanId;
    useEffect(() =>{
        callTauri<BudgetPlanModel>("get_one_budget_plan",{id:budgetPlanId}).then(result => setBudgetPlan(result));
    },[]);

    return(
        <>
            <h1>{budgetPlan.name}</h1>
            <BudgetPlanCategoryTable entry={budgetPlan}/>
            {budgetPlan.id && <BudgetPlanAddEdit entry={budgetPlan}/> }
        </>
    )
}

export default BudgetPlanView