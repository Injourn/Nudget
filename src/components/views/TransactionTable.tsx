import { MouseEventHandler, ReactNode, useEffect, useReducer, useState } from "react";
import TransactionViewDetail from "./TransactionViewDetail";
import TransactionResponseModel from "../../models/TransactionResponseModel";
import TransactionRequestModel from "../../models/TransactionRequestModel";
import TransactionRangeRequestModel from "../../models/TransactionRangeRequestModel";
import callTauri from "../../functions/CallTauri";
import BorderedWindow from "../uiElements/BorderedWindow";
import Table from "../uiElements/Table";
import Modal from "../uiElements/Modal";
import TransactionAddEdit from "../forms/TransactionAddEdit";

interface TransactionTableProps{
    startDate?:string,
    endDate?:string
}

function TransactionTable(props:TransactionTableProps){
    const defaultValue: string = getLocalDate();
    const [tableData, setTableData] = useState<TransactionResponseModel[]>([]);
    const [modalData, setModalData] = useState<TransactionRequestModel>({} as TransactionRequestModel);
    const [updater, forceUpdate] = useReducer(x => x + 1, 0);
    const columnNames:string[] = ["Amount","Categories","Date","Name"]

    useEffect(() => {
        if(props.startDate && props.endDate){
            let request:TransactionRangeRequestModel = {} as TransactionRangeRequestModel;
            request.start_date = props.startDate;
            request.end_date = props.endDate;
            callTauri<TransactionResponseModel[]>("get_transactions_in_range",{transactionRequest:request})
            .then(transaction => setTableData(transaction));
        } else {
            callTauri<TransactionResponseModel[]>("get_transaction").then(transactions => setTableData(transactions));
        }
    },[props,modalData,updater]);

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
            name: model.name,
            recurring: model.recurring,
            credit: model.credit}  as TransactionRequestModel
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
            name: "",
            recurring: false,
            credit: false}  as TransactionRequestModel)
        return undefined;
    }

    function onTransactionSubmit() {
        console.debug("submitted transaction");
        forceUpdate();
    }

    return (
        <BorderedWindow>
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
                <TransactionAddEdit modalName="transactionModalAddEdit" entry={modalData} onSubmit={onTransactionSubmit}/>
            </Modal>
        </BorderedWindow>
        
    )
}

export default TransactionTable;