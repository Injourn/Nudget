import { invoke } from "@tauri-apps/api/tauri";
import TransactionRequestModel from "../models/TransactionRequestModel";
import CategoryModel from "../models/CategoryModel";
import { useState } from "react";


function TransactionAddEdit(props: any){
    const item: TransactionRequestModel = props.entry;
    const [categories,setCategories]= useState<CategoryModel[]>([])
    invoke<CategoryModel[]>("get_all_categories").then(items => setCategories(items));

    function addTransaction(formData:React.SyntheticEvent){
        console.log(item);
        if(item.id){
            invoke("update_transaction",{transaction: item})
        }
        else {
            invoke("add_transaction",{transaction: item})
        }
        formData.preventDefault();
    }
    
    return(
        <form onSubmit={addTransaction}>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <label htmlFor="amount" className="col-form-label">
                        Amount
                    </label>
                </div>
                <div className="col-auto">
                    <input type="text" id="amount" className="form-control" value={item.amount} onChange={(e) => item.amount = e.target.value}/>
                </div>
            </div>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <label htmlFor="catgory" className="col-form-label">
                        Category
                    </label>
                </div>
                <div className="col-auto">
                    <select name="category" id="category" className="form-select" value={item.category_id} onChange={(e) => item.category_id = Number(e.target.value)}>
                        {categories.map((data,i) =>
                            <option value={data.id}>{data.name}</option>
                        )}
                    </select>
                </div>
            </div>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <label htmlFor="date" className="col-form-label">
                        Date
                    </label>
                </div>
                <div className="col-auto">
                    <input type="date" id="date" className="form-control" value={item.transaction_date} onChange={(e) => item.transaction_date = e.target.value}/>
                </div>
            </div>
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
    )
}

export default TransactionAddEdit;