import { useState, useEffect } from "react";
import callTauri from "../../functions/CallTauri";
import GenericForm from "./common/GenericForm";
import GenericFormInput from "./common/GenericFormInput";


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
    function onChange(e:any){
        const value = e.target.value;
        const object : Account = item;
        const name = e.target.name as keyof typeof object;
        object[name] = value;

        setItem({...item,object});
    }

    return (
        <GenericForm modalName={props.modalName} onSubmit={onSubmit} edit={item.id > 0} onRemove={removeAccount}>
            <GenericFormInput id={"name"} label={"Name"} item={item.name}
                type={"text"} onChange={onChange}/>
            <GenericFormInput onChange={onChange} 
                id={"currency"} label={"Currency Type"} item={item.currency_type} type={"text"} />
        </GenericForm>
    );
}

export default AccountAddEdit;