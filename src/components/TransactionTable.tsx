import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import TransactionModel from "../models/TransactionModel";
import Modal from "./Modal";
import TransactionViewDetail from "./TransactionViewDetail";
import TransactionAddEdit from "./TransactionAddEdit";

function TransactionTable(){
    const defaultValue: string = getLocalDate();
    const [tableData, setTableData] = useState<TransactionModel[]>([]);
    //TODO: Consider removing add state here.
    const [showAdd, setAdd] = useState<boolean>(false);
    const [modalData, setModalData] = useState<TransactionModel>({} as TransactionModel);

    invoke<TransactionModel[]>("get_transaction").then(transactions => setTableData(transactions))

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

    function changeModalData(model:TransactionModel){
        setModalData(model);
    }
    
    function removeItem(model: TransactionModel){
        invoke("remove_transaction",{transaction: model})
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
                            name: ""}  as TransactionModel)}>
                            +
                        </button>
                </th>
            </thead>
            <tbody>
            {tableData.map((data,i) => 
                <tr>
                    <td>{data.amount}</td>
                    <td>{data.category_id}</td>
                    <td>{data.transaction_date}</td>
                    <td>{data.name}</td>
                    <td>
                        <button type="button" className="btn btn-primary"
                        data-bs-toggle="modal" data-bs-target="#transactionModalView"
                        onClick={() => changeModalData(data)}>
                            {i}
                        </button>
                        <button type="button" className="btn btn-primary"
                        data-bs-toggle="modal" data-bs-target="#transactionModalAddEdit"
                        onClick={() => changeModalData(data)}>
                            {i}
                        </button>
                        <button type="button" className="btn btn-primary"
                        onClick={() => removeItem(data)}>
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