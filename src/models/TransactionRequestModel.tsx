interface TransactionRequestModel {
    id: number;
    amount: string;
    category_id: number;
    transaction_date: string;
    name: string;
    recurring: boolean;
}

export default TransactionRequestModel;