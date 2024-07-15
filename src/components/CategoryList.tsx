import { useEffect, useState } from "react";
import CategoryModel from "../models/CategoryModel";
import Modal from "./ui/Modal";
import CategoryAddEdit from "./CategoryAddEdit";
import callTauri from "../functions/CallTauri";
import DataList from "./ui/DataList";

function CategoryList(){
    const [modalData, setModalData] = useState<CategoryModel>({} as CategoryModel);
    const [tableData, setTableData] = useState<CategoryModel[]>([]);
    useEffect(() =>{
        callTauri<CategoryModel[]>("get_all_categories").then(categories => setTableData(categories));
    },[modalData]);
    
    function changeModalData(model:CategoryModel){
        setModalData(model);
    }

    function addRowClick(){
        changeModalData({id:0, name:""} as CategoryModel);
    }

    function onRowClick(data: CategoryModel){
        changeModalData(data);
    }
    
    function listRow(data:CategoryModel){
        return(
            <>{data.name}</>
        )
    }


    return(
        <>
            <b>Categories</b>
            <DataList 
             modalTarget={"categoryModelAddEdit"}
             addRowClick={addRowClick}
             listRow={listRow} 
             onRowClick={onRowClick} 
             listData={tableData}
             addRowBox={true}
             >
            </DataList>
            <Modal name="categoryModelAddEdit" title="Category">
                <CategoryAddEdit modalName="categoryModelAddEdit" category={modalData}/>
            </Modal>
        </>
    )
}

export default CategoryList;