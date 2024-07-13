import callTauri from "../functions/CallTauri";
import CategoryModel from "../models/CategoryModel";
import GenericForm from "./forms/GenericForm";
import GenericFormInput from "./forms/GenericFormInput";


function CategoryAddEdit(props:any){
    const item: CategoryModel = props.category;

    function onSubmit(formData:React.SyntheticEvent){
        console.log(item);
        if(item.id){
            callTauri("update_category",{category: item});
        }
        else {
            callTauri("add_category",{category: item})
        }
        formData.preventDefault();
    }
    function removeCategory(){
        callTauri("remove_category",{category:item})
    }

    return (
        <GenericForm modalName={props.modalName} onSubmit={onSubmit} edit={item.id > 0} onRemove={removeCategory}>
            <GenericFormInput id={"name"} label={"Name"} item={item.name}
                type={"text"} onChange={(e) => item.name = e.target.value}/>
        </GenericForm>
    );
}

export default CategoryAddEdit;