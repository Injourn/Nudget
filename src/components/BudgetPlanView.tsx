import { useState } from "react";
import BudgetPlanCategoryTable from "./BudgetPlanCategoryTable"
import BudgetPlanModel from "../models/BudgetPlanModel";
import { invoke } from "@tauri-apps/api";
import { useParams } from "react-router-dom";
import BudgetPlanAddEdit from "./BudgetPlanAddEdit";


function BudgetPlanView(props:any){
    let params = useParams();
    const [budgetPlan,setBudgetPlan] = useState<BudgetPlanModel>({} as BudgetPlanModel);
    let budgetPlanId = params.budgetPlanId ?? props.budgetPlanId;
    invoke<BudgetPlanModel>("get_one_budget_plan",{id:budgetPlanId}).then(result => setBudgetPlan(result));

    return(
        <>
            <h1>{budgetPlan.name}</h1>
            <BudgetPlanCategoryTable entry={budgetPlan}/>
            {budgetPlan.id && <BudgetPlanAddEdit entry={budgetPlan}/> }
        </>
    )
}

export default BudgetPlanView