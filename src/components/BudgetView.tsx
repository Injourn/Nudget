import { useEffect, useState } from "react";
import BudgetModel from "../models/BudgetModel";
import callTauri from "../functions/CallTauri";
import BudgetStatisticsView from "./BudgetStatisticsView";
import { useParams } from "react-router-dom";
import TransactionTable from "./TransactionTable";


function BudgetView(props:any){
    let params = useParams();
    let budgetId = params.budgetId ?? props.budgetId;
    let showTransactions = props.showTransactions ?? true;
    const [budget,setBudget] = useState<BudgetModel>({} as BudgetModel);
    useEffect(() => {
        callTauri<BudgetModel>("get_one_budget",{id:budgetId}).then(budget => setBudget(budget));
    },[]);

    return (
        <>
            <p>From: {budget.start_date} - {budget.end_date}</p>
            <BudgetStatisticsView entry={budget} />
            {showTransactions && <TransactionTable startDate={budget.start_date} endDate={budget.end_date} />}
        </>
    )
}

export default BudgetView;