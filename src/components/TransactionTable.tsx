import { MouseEventHandler, ReactNode, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import TransactionRequestModel from "../models/TransactionRequestModel";
import Modal from "./ui/Modal";
import TransactionViewDetail from "./TransactionViewDetail";
import TransactionAddEdit from "./TransactionAddEdit";
import TransactionResponseModel from "../models/TransactionResponseModel";
import Table from "./ui/Table";

function TransactionTable(){
    const defaultValue: string = getLocalDate();
    const [tableData, setTableData] = useState<TransactionResponseModel[]>([]);
    //TODO: Consider removing add state here.
    const [showAdd, setAdd] = useState<boolean>(false);
    const [modalData, setModalData] = useState<TransactionRequestModel>({} as TransactionRequestModel);
    const columnNames:string[] = ["Amount","Categories","Date","Name"]

    invoke<TransactionResponseModel[]>("get_transaction").then(transactions => setTableData(transactions))

    function getLocalDate() : string{
        const date:Date = new Date();
        const day : string = date.toLocaleString('en', {day: '2-digit'});   // DD
        const month : string = date.toLocaleString('en', {month: '2-digit'}); // MM
        const year : string = date.toLocaleString('en', {year: 'numeric'}); // YYYY
        return `${year}-${month}-${day}`
    }

    function changeModalData(model:TransactionRequestModel){
        setModalData(model);
    }

    function responseModelToRequestModel(model: TransactionResponseModel){

        return {id: model.id,
            amount: model.amount,
            category_id: model.category_id,
            transaction_date: model.transaction_date,
            name: model.name}  as TransactionRequestModel
    }
    function onRowClick(model: TransactionResponseModel){
        changeModalData(responseModelToRequestModel(model))
    }

    function tableRow(data:any): ReactNode{
        return (
            <>
                <td>{data.amount}</td>
                <td>{data.category_name}</td>
                <td>{data.transaction_date}</td>
                <td>{data.name}</td>
            </>
        )
    }

    function addRow() : MouseEventHandler<HTMLButtonElement> | undefined {
        changeModalData({id: 0,
            amount: "",
            category_id: 0,
            transaction_date: defaultValue,
            name: ""}  as TransactionRequestModel)
        return undefined;
    }

    return (
        <>
            <Table 
            tableRow={tableRow}
            tableData={tableData}
            columns={columnNames}
            modalTarget="transactionModalAddEdit"
            onRowClick={onRowClick}
            addRowClick={addRow}
            editable={true}
            addRowBox={true}>
            </Table>
            
            <Modal name="transactionModalView" title="View Transaction">
                <TransactionViewDetail entry={modalData} />
            </Modal>
            <Modal name="transactionModalAddEdit" title="Transaction">
                <TransactionAddEdit modalName="transactionModalAddEdit" entry={modalData}/>
            </Modal>
        </>
        
    )
}

export default TransactionTable;