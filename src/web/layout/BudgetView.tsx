import { useEffect, useState } from "react";
import BudgetStatisticsView from "./BudgetStatisticsView";
import { useParams } from "react-router-dom";
import callTauri from "../../functions/CallTauri";
import AccountView from "./AccountView";
import BudgetModel from "../../models/entity/BudgetModel";
import TransactionTable from "./TransactionTable";


function BudgetView(props:any){
    let params = useParams();
    let budgetId = params.budgetId ?? props.budgetId;
    let showTransactions = props.showTransactions ?? true;
    let budgetDate = props.budgetDateRange;
    
    const [budget,setBudget] = useState<BudgetModel>({} as BudgetModel);
    let startDate = budget?.start_date ?? props.startDate ?? budgetDate;
    let endDate = budget?.end_date ?? props.endDate ?? setAdjacentMonth(budgetDate);
    let accountSummaryRequest = {account_id: 1, start_date: startDate, end_date: endDate} as AccountSummaryRequest;
    useEffect(() => {
        if (budgetId) {
            callTauri<BudgetModel>("get_one_budget",{id:budgetId}).then(budget => setBudget(budget));
        } else if (budgetDate){
            console.debug(budgetDate)
            callTauri<BudgetModel>("get_one_budget_by_date",{range:budgetDate}).then(budget =>setBudget(budget));
        }
    },[props]);
    

    return (
        <>
            <p>From: {startDate} - {endDate}</p>
            <AccountView summaryRequest={accountSummaryRequest}/>
            <BudgetStatisticsView entry={budget} startDate={startDate} endDate={endDate}/>
            {showTransactions && <TransactionTable startDate={startDate} endDate={endDate} />}
        </>
    )
}

function setAdjacentMonth(date:string) : string{
    let newDate = new Date(date);
    newDate.setMonth(newDate.getMonth() + 1);
    newDate.setDate(newDate.getDate() - 1);
    return newDate.toISOString().split("T")[0];
}

export default BudgetView;