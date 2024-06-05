import Cycle from "./Cycle";


interface BudgetModel{
    id:number,
    start_date: String,
    cycle?: Cycle,
}

export default BudgetModel;