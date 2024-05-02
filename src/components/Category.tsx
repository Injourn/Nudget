import { invoke } from "@tauri-apps/api";
import CategoryModel from "../models/CategoryModel";
import "./Category.css"

function Category(props: any){

    function removeCategory(item: CategoryModel){
        invoke("remove_category",{category:item})
    }

    return(
        <li className="list-group-item">
            {props.data.name}
            <button type="button" className="btn btn-primary"
                data-bs-toggle="modal" data-bs-target="#categoryModelAddEdit"
                onClick={() => props.changeModalData(props.data)}>
                    Edit
            </button>    
            <button type="button" className="btn btn-primary"
                onClick={() => removeCategory(props.data)}>
                    Remove
            </button>
        </li>
    );
}

export default Category

