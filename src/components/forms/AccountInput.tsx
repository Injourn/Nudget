import { useEffect, useState } from "react";
import GenericSelectInput from "./generic/GenericSelectInput";
import callTauri from "../../functions/CallTauri";

interface AccountInputProps{
    setItem: (arg0: any) => void;
    item: { account_id: unknown; };
 }

function AccountInput(props:AccountInputProps){
    const [accounts,setAccounts]= useState<Account[]>([]);

    useEffect(() => {
        callTauri<Account[]>("get_all_account").then(items => setAccounts(items));
    }, []);

    return <GenericSelectInput onChange={(e) => props.setItem({...props.item,account_id: Number(e.target.value)})} id={"category"}
             label={"Account"} item={props.item.account_id}>
                {accounts.map((data) =>
                    <option value={data.id}>{data.name}</option>
                )}
            </GenericSelectInput>

}

export default AccountInput;