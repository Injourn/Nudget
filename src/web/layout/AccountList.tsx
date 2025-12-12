import {useEffect, useState } from "react";
import DataList from "../elements/DataList";
import callTauri from "../../functions/CallTauri";
import AccountAddEdit from "../forms/AccountAddEdit";
import Modal from "../elements/Modal";


function AccountList(){
    const [modalData, setModalData] = useState<Account>({id:0,name:"",created_date:"",currency_type:""} as Account);
    const [tableData, setTableData] = useState<Account[]>([]);
    const [testValue, setTestValue] = useState<number>();
    useEffect(() =>{
        callTauri<Account[]>("get_all_account").then(categories => setTableData(categories));
    },[modalData]);
    useEffect(() => {
        callTauri<number>("get_account_net_value",{id:"1"}).then(item => setTestValue(item));
    },[testValue])
    console.log(testValue)
    function changeModalData(model:Account){
        setModalData(model);
    }

    function addRowClick(){
        changeModalData({id:0, name:"", currency_type:"", created_date:""} as Account);
    }

    function onRowClick(data: Account){
        changeModalData(data);
    }

    function listRow(data:Account){
        return (
        <>
            {data.name}
            &emsp;
            {data.currency_type}
        </>
        )
    }

    

    return <>
            <DataList 
                modalTarget={"accountModelAddEdit"} 
                addRowClick={addRowClick}
                onRowClick={onRowClick}
                listRow={listRow}
                listData={tableData}
                addRowBox={true}>
            </DataList>
            <Modal name="accountModelAddEdit" title="Account">
                <AccountAddEdit modalName="accountModelAddEdit" account={modalData} onSubmit={() => {}}/>
            </Modal>
        </>
}

export default AccountList;