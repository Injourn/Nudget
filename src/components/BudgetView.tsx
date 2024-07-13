import { useState } from "react";
import BudgetModel from "../models/BudgetModel";
import callTauri from "../functions/CallTauri";
import BudgetStatisticsView from "./BudgetStatisticsView";
import { useParams } from "react-router-dom";


function BudgetView(props:any){
    let params = useParams();
    let budgetId = params.budgetId ?? props.budgetId;
    const [budget,setBudget] = useState<BudgetModel>({} as BudgetModel);

    callTauri<BudgetModel>("get_one_budget",{id:budgetId}).then(budget => setBudget(budget));

    return (
        <>
            <p>From: {budget.start_date} - {budget.end_date}</p>
            <BudgetStatisticsView entry={budget} />
        </>
    )
}

export default BudgetView;