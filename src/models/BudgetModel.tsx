import Cycle from "./Cycle";


interface BudgetModel{
    id:number,
    start_date: String,
    cycle?: Cycle,
    end_date: String
}

export default BudgetModel;