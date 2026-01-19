import Cycle from "../entity/Cycle";


interface TransactionRequestModel {
    id: number;
    amount: string;
    category_name: string;
    category_id: number
    transaction_date: string;
    name: string;
    recurring:boolean;
    credit:boolean;
    cycle:Cycle;
    day_of_month:number;
    day_of_week:number;
    account_id:number;
    account_name:string;
}

export default TransactionRequestModel;