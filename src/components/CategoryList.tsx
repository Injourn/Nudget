import { useState } from "react";
import CategoryModel from "../models/CategoryModel";
import Category from "./Category";

function CategoryList(){
    const [tableData, setTableData] = useState<CategoryModel[]>([
        {
            name: "Rent",
            childCategories: [],
        },
        {
            name: "Groceries",
            childCategories: [],
        },
        {
            name: "OtherIncome",
            childCategories: [{
                name: "Restaurants",
                childCategories: [],
            },
            {
                name: "Extra",
                childCategories: [
                    {
                        name: "Extra 2",
                        childCategories: [],
                    }
                ],
            },
            {
                name: "Rent",
                childCategories: [],
            }],
        },
        {
            name: "Savings",
            childCategories: [],
        },
    ]);
    


    return(
        <ul className="list-group list-group-flush">
            {tableData.map((data) => 
                <Category text={data.name} children={data.childCategories} tabs=""/>
            )}
        </ul>
    )
}

export default CategoryList;