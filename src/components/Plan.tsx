import { useState } from "react";
import BudgetCategoryModel from "../models/BudgetCategoryModel";
import { invoke } from "@tauri-apps/api";
import Modal from "./ui/Modal";
import BudgetCategoryAddEdit from "./BudgetCategoryAddEdit";



function Plan(){
    const [modalData,setModalData] = useState<BudgetCategoryModel[]>([]); 
    const [requestData,setRequestData] = useState<BudgetCategoryModel>({} as BudgetCategoryModel);

    invoke<BudgetCategoryModel[]>("get_all_budget_categories").then(result => setModalData(result));


    function changeModalData(model:BudgetCategoryModel){
        setRequestData(model);
    }

    return (
        <>
            <p>Date Range: 03/15/2024 - 04/15/2024</p>
            <p style={{color:"green"}}>800</p>
            {modalData.map((data,i) => 
                <p>{data.category_id} {data.flat_amount}</p>
            )}
            <button type="button" className="btn btn-primary"
                         data-bs-toggle="modal" data-bs-target="#budgetCategoryModalAddEdit"
                         onClick={() => changeModalData({id: 0,
                            flat_amount: "",
                            category_id: 0,
                            percentage_amount:""
                            }  as BudgetCategoryModel)}>
                            +
                        </button>
            <Modal name="budgetCategoryModalAddEdit" title="Add/Edit Budget Category">
              <BudgetCategoryAddEdit entry={requestData} />
            </Modal>
        </>
    )
}

export default Plan;