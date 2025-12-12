import Cycle from "./Cycle";


interface BudgetModel{
    id:number,
    start_date: string,
    cycle?: Cycle,
    end_date: string
}

export default BudgetModel;