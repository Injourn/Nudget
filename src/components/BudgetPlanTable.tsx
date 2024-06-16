import { MouseEventHandler, ReactNode, useState } from "react";
import Table from "./ui/Table";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api";
import BudgetPlanModel from "../models/BudgetPlanModel";
import Cycle from "../models/Cycle";


function BudgetPlanTable(){
    const [tableData,setTableData] = useState<BudgetPlanModel[]>([]);
    const columns = ["Table Name", "Cycle","Start Date/Day"];
    const navigate = useNavigate();
    invoke<BudgetPlanModel[]>("get_all_budget_plan").then(result => setTableData(result));

    function tableRow(data: BudgetPlanModel): ReactNode {
        
        return(
            <>
                <td></td>
                <td>{data.cycle}</td>
                <td>{data.cycle == Cycle.MONTHLY ? data.start_date_of_month : numberToDayOfWeek(data.start_date_of_week ?? -1)}</td>
            </>
        )
    }

    function numberToDayOfWeek(dayOfWeek:number) : String {
        let day:String;
        switch(dayOfWeek){
            case 0:
                day = "Sunday";
                break;
            case 1:
                day = "Monday";
                break;
            case 2:
                day = "Tuesday";
                break;
            case 3:
                day = "Wednesday";
                break;
            case 4:
                day = "Thursday";
                break;
            case 5:
                day = "Friday";
                break;
            case 6:
                day = "Saturday";
                break;
            default:
                day = "Error";
        }
        return day;
    }

    function onRowClick(data: BudgetPlanModel){
        navigate("/budgetPlan/" + data.id);
    }

    function addRowClick(): MouseEventHandler<HTMLButtonElement> | undefined {
        navigate("/budgetPlan/new");
        return undefined;
    }

    return(
        <Table 
         tableRow={tableRow} 
         tableData={tableData}
         columns={columns}
         onRowClick={onRowClick}
         addRowClick={addRowClick}
         addRowBox={true}
         >
            
        </Table>
    )
}

export default BudgetPlanTable;