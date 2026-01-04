import { useEffect, useState } from "react";
import ProgressBar from "../elements/ProgressBar";
import callTauri from "../../functions/CallTauri";

interface AccountViewProps{
    summaryRequest: AccountSummaryRequest
}

function AccountView(props:AccountViewProps){
    // let [accountSummary,setAccountSummary] = useState<AccountSummaryResponse>({} as AccountSummaryResponse)
    let [summaryValue,setSummaryValue] = useState<number>(0);
    
    useEffect(() => {
        callTauri<AccountSummaryResponse>("get_account_summary_in_range", {accountRequest:props.summaryRequest})
        .then(summary => {
                // setAccountSummary(summary)
                if(summary){
                let percentageRemaining = (summary.credit_transactions - summary.debit_transactions) / summary.credit_transactions * 100;
                if(percentageRemaining > 0 && percentageRemaining <= 100){
                    setSummaryValue(percentageRemaining);
                } else {
                    console.warn("Did not update summary value due to invalid value: " + percentageRemaining)
                }
                console.log("Calculated the net sumary value to be: " + (summary.credit_transactions - summary.debit_transactions).toString() + "(" + summaryValue + "%)");
                }
            }
        );
    }, [props])
    return (
    <div>
        <ProgressBar value={summaryValue}/>
    </div>);
}

export default AccountView;