import { MouseEventHandler, ReactNode, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import BudgetPlanModel from "../../models/BudgetPlanModel";
import callTauri from "../../functions/CallTauri";
import Cycle from "../../models/Cycle";
import Table from "../elements/Table";
import numberToDayOfWeek from "../../functions/DateModifcations";


function BudgetPlanTable(){
    const [tableData,setTableData] = useState<BudgetPlanModel[]>([]);
    const columns = ["Table Name", "Cycle","Start Date/Day"];
    const navigate = useNavigate();
    useEffect(() => {
        callTauri<BudgetPlanModel[]>("get_all_budget_plan").then(result => setTableData(result));
    },[])
    

    function tableRow(data: BudgetPlanModel): ReactNode {
        
        return(
            <>
                <td>{data.name}</td>
                <td>{data.cycle}</td>
                <td>{data.cycle == Cycle.MONTHLY ? data.start_date_of_month : numberToDayOfWeek(data.start_date_of_week ?? -1)}</td>
            </>
        )
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