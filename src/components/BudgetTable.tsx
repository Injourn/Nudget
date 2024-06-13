import { ReactNode, useState } from "react";
import Table from "./ui/Table";
import BudgetModel from "../models/BudgetModel";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api";


function BudgetTable(){
    const [tableData,setTableData] = useState<BudgetModel[]>([]);
    const columns = ["Table Name", "Date"];
    const navigate = useNavigate();
    invoke<BudgetModel[]>("get_all_budget").then(result => setTableData(result));

    function tableRow(data: BudgetModel): ReactNode {
        return(
            <>
                <td></td>
                <td>{data.start_date} - {data.end_date}</td>
            </>
        )
    }

    function onRowClick(data: BudgetModel){
        navigate("/budget/" + data.id);
    }

    return(
        <Table 
         tableRow={tableRow} 
         tableData={tableData}
         columns={columns}
         onRowClick={onRowClick}
         >
            
        </Table>
    )
}

export default BudgetTable;