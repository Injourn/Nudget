import { useState } from "react";
import CategoryModel from "../models/CategoryModel";
import Category from "./Category";
import Modal from "./Modal";
import CategoryAddEdit from "./CategoryAddEdit";

function CategoryList(){
    const [modalData, setModalData] = useState<CategoryModel>({} as CategoryModel);
    const [tableData, setTableData] = useState<CategoryModel[]>([
        {
            id:0,
            name: "Rent",
        },
        {
            id:1,
            name: "Groceries",
        },
        {
            id:2,
            name: "OtherIncome",
        },
        {
            id:7,
            name: "Savings",
        },
    ]);
    
    function changeModalData(model:CategoryModel){
        setModalData(model);
    }


    return(
        <ul className="list-group list-group-flush">
            {tableData.map((data,i) => 
            
                <Category data={data} key={i} changeModalData={changeModalData}/>
            )}
            <button type="button" className="btn btn-primary"
                data-bs-toggle="modal" data-bs-target="#categoryModelAddEdit"
                onClick={() => changeModalData({} as CategoryModel)}>
                    +
            </button>
            <Modal name="categoryModelAddEdit" title="Add Category">
                <CategoryAddEdit category={modalData}/>
            </Modal>
        </ul>
    )
}

export default CategoryList;