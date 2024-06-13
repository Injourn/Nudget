import { MouseEventHandler, useState } from "react";
import BudgetCategoryModel from "../models/BudgetCategoryModel";
import { invoke } from "@tauri-apps/api";
import Table from "./ui/Table";
import Modal from "./ui/Modal";
import BudgetCategoryAddEdit from "./BudgetCategoryAddEdit";
import BudgetModel from "../models/BudgetModel";

interface BudgetCategoryTableProps{
    entry:BudgetModel;
}


function BudgetCategoryTable(props:BudgetCategoryTableProps){
    const [itemData,setItemData] = useState<BudgetCategoryModel[]>([]);
    const columns = ["Category","Amount"];
    const [modalData,setModalData] = useState<BudgetCategoryModel>(defaultModalData());

    invoke<BudgetCategoryModel[]>("get_all_budget_budget_categories",{budget:props.entry}).then(model => setItemData(model))

    function tableRow(data:BudgetCategoryModel){
        return(
            <>
                <td>{data.category_id}</td>
                <td>{data.flat_amount}</td>
            </>
        )
    }
    
    function onRowClick(data:BudgetCategoryModel){
        setModalData(data);
    }

    function defaultModalData(): BudgetCategoryModel{
        return {id: 0,
            category_id: 0,
            flat_amount: "",
            fixed:false,
            percentage_amount:""
        } as BudgetCategoryModel;
    }


    function addRowClick(): MouseEventHandler<HTMLButtonElement> | undefined {
        setModalData(defaultModalData())
        return undefined;
    }

    function addBudgetBudgetCategory(budgetCategoryId:number){
        console.log(budgetCategoryId)
        invoke("add_budget_budget_category",{budgetCategoryId:budgetCategoryId,budgetId:props.entry.id})
    }

    return(
        <>
            <Table 
             tableRow={tableRow}
             tableData={itemData}
             columns={columns}
             addRowBox={true}
             addRowClick={addRowClick}
             onRowClick={onRowClick}
             editable={true}
             modalTarget="budgetCategoryModal"
             >
            </Table>
            <Modal name={"budgetCategoryModal"} title={"Budget Category"}>
                <BudgetCategoryAddEdit entry={modalData} parentAdd={addBudgetBudgetCategory}/>
            </Modal>
        </>
    )
}

export default BudgetCategoryTable;