import { invoke } from "@tauri-apps/api";
import CategoryModel from "../models/CategoryModel";
import GenericForm from "./forms/GenericForm";
import GenericFormInput from "./forms/GenericFormInput";


function CategoryAddEdit(props:any){
    const item: CategoryModel = props.category;

    function onSubmit(formData:React.SyntheticEvent){
        console.log(item);
        if(item.id){
            invoke("update_category",{category: item});
        }
        else {
            invoke("add_category",{category: item})
        }
        formData.preventDefault();
    }

    return (
        <form onSubmit={onSubmit}>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <label htmlFor="name" className="col-form-label">
                        Name
                    </label>
                </div>
                <div className="col-auto">
                    <input type="name" id="text" className="form-control" value={item.name} onChange={(e) => item.name = e.target.value}/>
                </div>
            </div>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <input type="submit" className="btn" data-bs-dismiss="modal" value={item.id ? "Edit Entry" : "Add Entry"} />
                </div>
            </div>
        </form>
    );
}

export default CategoryAddEdit;