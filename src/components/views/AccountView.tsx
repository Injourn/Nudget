import { useEffect, useState } from "react";
import ProgressBar from "../uiElements/ProgressBar";
import callTauri from "../../functions/CallTauri";

interface AccountViewProps{
    summaryRequest: AccountSummaryRequest
}

function AccountView(props:AccountViewProps){
    let [accountSummary,setAccountSummary] = useState<AccountSummaryResponse>({} as AccountSummaryResponse)
    let differencePercentage = 0;
    useEffect(() => {
        callTauri<AccountSummaryResponse>("get_account_summary_in_range", {accountRequest:props.summaryRequest})
        .then(summary => setAccountSummary(summary))
        if(accountSummary){
            console.log(accountSummary);
            differencePercentage = (6000 - accountSummary.debit_transactions)
            console.log(accountSummary.debit_transactions);
            console.log(differencePercentage);
        }
    }, [props])
    return (
    <div>
        <ProgressBar value={50}/>
    </div>);
}

export default AccountView;