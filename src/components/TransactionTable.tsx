import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import TransactionRequestModel from "../models/TransactionRequestModel";
import Modal from "./ui/Modal";
import TransactionViewDetail from "./TransactionViewDetail";
import TransactionAddEdit from "./TransactionAddEdit";
import TransactionResponseModel from "../models/TransactionResponseModel";

function TransactionTable(){
    const defaultValue: string = getLocalDate();
    const [tableData, setTableData] = useState<TransactionResponseModel[]>([]);
    //TODO: Consider removing add state here.
    const [showAdd, setAdd] = useState<boolean>(false);
    const [modalData, setModalData] = useState<TransactionRequestModel>({} as TransactionRequestModel);

    invoke<TransactionResponseModel[]>("get_transaction").then(transactions => setTableData(transactions))

    function getLocalDate() : string{
        const date:Date = new Date();
        const day : string = date.toLocaleString('en', {day: '2-digit'});   // DD
        const month : string = date.toLocaleString('en', {month: '2-digit'}); // MM
        const year : string = date.toLocaleString('en', {year: 'numeric'}); // YYYY
        return `${year}-${month}-${day}`
    }
    // function onChangeModel(item: any){
    //     setModalData(model => item)
    // }

    function changeModalData(model:TransactionRequestModel){
        setModalData(model);
    }
    
    function removeItem(model: TransactionRequestModel){
        invoke("remove_transaction",{transaction: model})
    }

    function responseModelToRequestModel(model: TransactionResponseModel){

        return {id: model.id,
            amount: model.amount,
            category_id: model.category_id,
            transaction_date: model.transaction_date,
            name: model.name}  as TransactionRequestModel
    }
    return (
        <table className="table table-striped">
            <thead>
                <th scope="col">Amount</th>
                <th scope="col">Category</th>
                <th scope="col">Date</th>
                <th scope="col">Name</th>
                <th scope="col">
                        <button type="button" className="btn btn-primary"
                         data-bs-toggle="modal" data-bs-target="#transactionModalAddEdit"
                         onClick={() => changeModalData({id: 0,
                            amount: "",
                            category_id: 0,
                            transaction_date: defaultValue,
                            name: ""}  as TransactionRequestModel)}>
                            +
                        </button>
                </th>
            </thead>
            <tbody>
            {tableData.map((data,i) => 
                <tr>
                    <td>{data.amount}</td>
                    <td>{data.category_name}</td>
                    <td>{data.transaction_date}</td>
                    <td>{data.name}</td>
                    <td>
                        <button type="button" className="btn btn-primary"
                        data-bs-toggle="modal" data-bs-target="#transactionModalView"
                        onClick={() => changeModalData(responseModelToRequestModel(data))}>
                            {i}
                        </button>
                        <button type="button" className="btn btn-primary"
                        data-bs-toggle="modal" data-bs-target="#transactionModalAddEdit"
                        onClick={() => changeModalData(responseModelToRequestModel(data))}>
                            {i}
                        </button>
                        <button type="button" className="btn btn-primary"
                        onClick={() => removeItem(responseModelToRequestModel(data))}>
                            {i}
                        </button>
                    </td>
                </tr>
                )
            }
            <Modal name="transactionModalView" title="View Transaction">
              <TransactionViewDetail entry={modalData} />
            </Modal>
            <Modal name="transactionModalAddEdit" title="Add Transaction">
                <TransactionAddEdit entry={modalData}/>
            </Modal>
            </tbody>
        </table>
)
}

{/* <button type="button" className="btn btn-primary" data-bs-toggle="modal" data-bs-target="#transactionModal">
              Launch demo modal
            </button>
            <Modal name="transactionModal" title="AddTransaction">
              <TransactionAddEdit />
            </Modal> */}

export default TransactionTable;