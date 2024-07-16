import BudgetModel from "../models/BudgetModel";
import { ReactNode, useEffect, useState } from "react";
import callTauri from "../functions/CallTauri";
import Table from "./ui/Table";
import BudgetStatisticsResponseModel from "../models/BudgetStatisticsResponseModel";
import BorderedWindow from "./ui/BorderedWindow";
import ProgressBar from "./ui/ProgressBar";


interface BudgetStatisticsViewProps{
    entry:BudgetModel,
}

function BudgetStatisticsView(props:BudgetStatisticsViewProps){
    const budget:BudgetModel = props.entry;
    const [budgetStatistics,setBudgetStatistics] = useState<BudgetStatisticsResponseModel[]>([]);
    const categoryNames:string[] = ["Category","Remaining","Budgeted"];
    useEffect(() => {
        callTauri<BudgetStatisticsResponseModel[]>("get_active_budget_statistics",{budget:budget}).then(categories => setBudgetStatistics(categories));
    },[props]);
    function tableRow(data:BudgetStatisticsResponseModel): ReactNode{
        let spentNumber = Number(data.category_spent);
        let budgetNumber = Number(data.category_budget);
        return(
            <>
                <td>{data.category_name} <br/> <ProgressBar value={spentNumber/budgetNumber*100}/></td>
                <td>{budgetNumber - spentNumber}</td>
                <td>{data.category_budget}</td>
            </>
            );
    }

    return (
        <BorderedWindow>
            <div className="row d-none d-md-flex">
                <div className="col-3">
                    <img width={200} height={200}></img>
                </div>
                <div className="col-9">
                    <Table 
                        tableRow={tableRow} 
                        tableData={budgetStatistics} 
                        columns={categoryNames} 
                        />
                </div>
            </div>
            <div className="d-flex d-md-none">
            <Table 
                        tableRow={tableRow} 
                        tableData={budgetStatistics} 
                        columns={categoryNames} 
                        />
            </div>
        </BorderedWindow>
    );
}


export default BudgetStatisticsView;