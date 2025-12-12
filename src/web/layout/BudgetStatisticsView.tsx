import { ReactNode, useEffect, useState } from "react";
import {Pie } from "react-chartjs-2";
import BudgetModel from "../../models/BudgetModel";
import BudgetStatisticsResponseModel from "../../models/response/BudgetStatisticsResponseModel";
import callTauri from "../../functions/CallTauri";
import ProgressBar from "../elements/ProgressBar";
import BorderedWindow from "../elements/BorderedWindow";
import Table from "../elements/Table";


interface BudgetStatisticsViewProps{
    entry?:BudgetModel,
    startDate?:string,
    endDate?:string
}

function BudgetStatisticsView(props:BudgetStatisticsViewProps){
    const budget:BudgetModel | undefined = props.entry;
    const [budgetStatistics,setBudgetStatistics] = useState<BudgetStatisticsResponseModel[]>([]);
    const categoryNames:string[] = ["Category","Remaining","Budgeted"];
    useEffect(() => {

        if(!budget?.id){
            callTauri<BudgetStatisticsResponseModel[]>("get_default_budget_statistics",{range:{start_date: props.startDate,end_date:props.endDate}}).then(categories => setBudgetStatistics(categories))
        } else {
            callTauri<BudgetStatisticsResponseModel[]>("get_active_budget_statistics",{budget:budget}).then(categories => setBudgetStatistics(categories))
        }
    },[props]);
    function tableRow(data:BudgetStatisticsResponseModel): ReactNode{
        let spentNumber = Number(data.category_spent);
        let budgetNumber = Number(data.category_budget);
        return(
            <>
                <td>{data.category_name} <br/> <ProgressBar value={spentNumber/budgetNumber*100}/></td>
                <td>{Math.round(((budgetNumber - spentNumber) + Number.EPSILON) * 100) / 100 }</td>
                <td>{data.category_budget}</td>
            </>
            );
    }
    
    return (
        <BorderedWindow>
            <div className="row d-none d-md-flex">
                <div className="col-3">
                    <Pie data={{
                            labels : budgetStatistics.map((data) => data.category_name),
                            datasets : [
                                {
                                    label: "Amount spent",
                                    backgroundColor: [
                                        "rgb(189, 43, 32)",
                                        "rgb(13, 5, 171)",
                                        "rgb(201, 197, 2)",
                                        "rgb(35, 212, 19)",
                                        "rgb(122, 13, 217)",
                                        ],
                                    borderColor: "rgb(255,255,255)",
                                    data: budgetStatistics.map((data) => Number(data.category_spent))
                                }
                            ]
                        }}
                    />
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