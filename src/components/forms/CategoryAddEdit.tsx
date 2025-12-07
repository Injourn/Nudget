import { useEffect, useState } from "react";
import callTauri from "../../functions/CallTauri";
import CategoryModel from "../../models/CategoryModel";
import GenericForm from "./common/GenericForm";
import GenericFormInput from "./common/GenericFormInput";


function CategoryAddEdit(props:any){
    const [item,setItem] = useState<CategoryModel>({} as CategoryModel);
    
    useEffect(() => {
        setItem(props.category);
    },[props.category]);

    function onSubmit(){
        console.debug(item);
        if(item.id){
            callTauri("update_category",{category: item});
        }
        else {
            callTauri("add_category",{category: item})
        }
        props.onSubmit();
        event?.preventDefault();
    }
    function removeCategory(){
        props.onSubmit();
        callTauri("remove_category",{category:item})
    }

    return (
        <GenericForm modalName={props.modalName} onSubmit={onSubmit} edit={item.id > 0} onRemove={removeCategory}>
            <GenericFormInput id={"name"} label={"Name"} item={item.name}
                type={"text"} onChange={(e) => setItem({...item,name: e.target.value})}/>
        </GenericForm>
    );
}

export default CategoryAddEdit;