import BudgetModel from "../models/BudgetModel";
import { ReactNode, useEffect, useState } from "react";
import callTauri from "../functions/CallTauri";
import Table from "./ui/Table";
import BudgetStatisticsResponseModel from "../models/BudgetStatisticsResponseModel";
import BorderedWindow from "./ui/BorderedWindow";
import ProgressBar from "./ui/ProgressBar";
import {Line, Pie } from "react-chartjs-2";
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
    ArcElement,
  } from 'chart.js';


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
                <td>{budgetNumber - spentNumber}</td>
                <td>{data.category_budget}</td>
            </>
            );
    }
    ChartJS.register(
        CategoryScale,
        ArcElement,
        PointElement,
        LineElement,
        Title,
        Tooltip,
        Legend
      );
    
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