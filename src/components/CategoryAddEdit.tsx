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
        <GenericForm onSubmit={onSubmit} edit={item.id > 0}>
            <GenericFormInput id={"name"} label={"Name"} item={item.name}
                type={"text"} onChange={(e) => item.name = e.target.value}/>
        </GenericForm>
    );
}

export default CategoryAddEdit;