import Cycle from "./Cycle";

interface BudgetPlanModel{
    id: number;
    cycle: Cycle;
    StartDayOfMonth: number;
    StartDayOfWeek: number;
}

export default BudgetPlanModel;