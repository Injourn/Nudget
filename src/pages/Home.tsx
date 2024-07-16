import BudgetView from "../components/BudgetView";
import TransactionTable from "../components/TransactionTable";
import Calendar from "../components/ui/Calendar";



function Home(){
    return(
        <>
            <Calendar />
            <BudgetView budgetId="1"/>
            <TransactionTable />
        </>
    )
}

export default Home;