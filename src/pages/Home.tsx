import BudgetView from "../components/BudgetView";
import TransactionTable from "../components/TransactionTable";



function Home(){
    return(
        <>
            <BudgetView budgetId="1"/>
            <TransactionTable />
        </>
    )
}

export default Home;