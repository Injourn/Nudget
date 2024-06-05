import { useState } from "react";
import BudgetModel from "../models/BudgetModel";
import { invoke } from "@tauri-apps/api";
import BudgetStatisticsView from "./BudgetStatisticsView";



function BudgetView(){
    const [budget,setBudget] = useState<BudgetModel>({} as BudgetModel);

    invoke<BudgetModel>("get_one_budget",{id:"1"}).then(budget => setBudget(budget));

    return (
        <>
            <p>From: {budget.start_date} - {budget.end_date}</p>
            <BudgetStatisticsView entry={budget}>

            </BudgetStatisticsView>
        </>
    )
}

export default BudgetView;