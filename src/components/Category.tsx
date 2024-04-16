import CategoryModel from "../models/CategoryModel";
import "./Category.css"

function Category(props: any){

    return(
        <li className="list-group-item">{props.data.name}</li>
    );
}

export default Category

