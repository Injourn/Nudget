import GenericForm from "./forms/GenericForm";
import GenericFormInput from "./forms/GenericFormInput";


function CategoryAddEdit(props:any){

    function onSubmit(formData:React.SyntheticEvent){
        formData.preventDefault();

    }

    return (
        <GenericForm onSubmit={onSubmit} >
            <GenericFormInput id="category" label="Category Name" item={props.category.name} type="text"/>
        </GenericForm>
    );
}

export default CategoryAddEdit;