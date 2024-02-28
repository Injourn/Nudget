import CategoryModel from "../models/CategoryModel";
import "./Category.css"

function Category(props: any){

    if(props.children){
        return(
            <>
            <li className="list-group-item">{props.tabs}{props.text}</li>
            <ul className="list-group list-group-flush sublist">
                    {props.children.map((data: CategoryModel) => 
                        <Category text={data.name} children={data.childCategories} tabs={props.tabs}/>
                    )}
                </ul>
            </>
        );
    }

    return(
        <li className="list-group-item">{props.text}</li>
    );
}

export default Category

