import BudgetModel from "../models/BudgetModel";
import { ReactNode, useState } from "react";
import { invoke } from "@tauri-apps/api";
import Table from "./ui/Table";
import BudgetStatisticsResponseModel from "../models/BudgetStatisticsResponseModel";


interface BudgetStatisticsViewProps{
    entry:BudgetModel,
}

function BudgetStatisticsView(props:BudgetStatisticsViewProps){
    const budget:BudgetModel = props.entry;
    const [budgetStatistics,setBudgetStatistics] = useState<BudgetStatisticsResponseModel[]>([]);
    const categoryNames:string[] = ["Category","Amount budgeted","Amount remaining"];

    invoke<BudgetStatisticsResponseModel[]>("get_active_budget_statistics",{budget:budget}).then(categories => setBudgetStatistics(categories));
    console.log(budgetStatistics)
    function tableRow(data:BudgetStatisticsResponseModel): ReactNode{
        return(
            <>
                <td>{data.category_name}</td>
                <td>{data.category_budget}</td>
                <td>{Number(data.category_budget) - Number(data.category_spent)}</td>
            </>
            );
    }

    return (
        <Table 
            tableRow={tableRow} 
            tableData={budgetStatistics} 
            columns={categoryNames} 
            />
    );
}


export default BudgetStatisticsView;