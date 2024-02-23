import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface ListingModel {
    amount: string;
    category: string;
    date: string;
    name: string;
}

function Listing(){
    const [tableData, setTableData] = useState<ListingModel[]>([]);

    console.log("hit")
    invoke<ListingModel[]>("listing").then(listings => setTableData(listings))

    return (
        <table className="table table-striped">
            <thead>
                <th>Amount</th>
                <th>Category</th>
                <th>Date</th>
                <th>Name</th>
            </thead>
            <tbody>
            {tableData.map((data) => 
                <tr>
                    <td>{data.amount}</td>
                    <td>{data.category}</td>
                    <td>{data.date}</td>
                    <td>{data.name}</td>
                </tr>
                )
            }
            </tbody>
        </table>
)
}

export default Listing;