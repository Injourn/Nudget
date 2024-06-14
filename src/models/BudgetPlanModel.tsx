import Cycle from "./Cycle";

interface BudgetPlanModel{
    id: number;
    cycle: Cycle;
    start_date_of_month?: number;
    start_date_of_week?: number;
}

export default BudgetPlanModel;