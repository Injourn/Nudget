import BudgetView from "../components/BudgetView";
import TransactionTable from "../components/TransactionTable";
import Calendar from "../components/ui/Calendar";



function Home(){
    return(
        <>
            <Calendar />
            <BudgetView budgetId="1"/>
            <TransactionTable startDate="2024-06-04" endDate="2024-06-08" />
        </>
    )
}

export default Home;