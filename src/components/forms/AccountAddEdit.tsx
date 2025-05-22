import { useState, useEffect } from "react";
import callTauri from "../../functions/CallTauri";
import GenericForm from "./generic/GenericForm";
import GenericFormInput from "./generic/GenericFormInput";


function AccountAddEdit(props:any){
    const [item,setItem] = useState<Account>({} as Account);
    
    useEffect(() => {
        setItem(props.account);
    },[props.account]);

    function onSubmit(){
        console.debug(item);
        if(item.id){
            callTauri("update_account",{account: item});
        }
        else {
            callTauri("add_account",{account: item})
        }
        props.onSubmit();
        event?.preventDefault();
    }
    function removeAccount(){
        props.onSubmit();
        callTauri("remove_account",{account:item})
    }

    return (
        <GenericForm modalName={props.modalName} onSubmit={onSubmit} edit={item.id > 0} onRemove={removeAccount}>
            <GenericFormInput id={"name"} label={"Name"} item={item.name}
                type={"text"} onChange={(e) => setItem({...item,name: e.target.value})}/>
            <GenericFormInput onChange={(e) => setItem({...item,currency_type: e.target.value})} 
                id={"currency"} label={"Currency Type"} item={item.currency_type} type={"text"} />
        </GenericForm>
    );
}

export default AccountAddEdit;