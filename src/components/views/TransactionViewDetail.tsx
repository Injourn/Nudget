import TransactionRequestModel from "../../models/TransactionRequestModel";


function TransactionViewDetail(props:any){
    const transactionModel:TransactionRequestModel = props.entry;


    return (
        <div>
            <div className="row align-items-center mb-3">
                    <div className="col-auto">
                        <label htmlFor="amount" className="col-form-label">
                            Amount
                        </label>
                    </div>
                    <div className="col-auto">
                        {transactionModel && transactionModel.amount}
                    </div>
                </div>
                <div className="row align-items-center mb-3">
                    <div className="col-auto">
                        <label htmlFor="catgory" className="col-form-label">
                            Category
                        </label>
                    </div>
                    <div className="col-auto">
                        {transactionModel && transactionModel.category_id}
                    </div>
                </div>
                <div className="row align-items-center mb-3">
                    <div className="col-auto">
                        <label htmlFor="date" className="col-form-label">
                            Date
                        </label>
                    </div>
                    <div className="col-auto">
                        {transactionModel && transactionModel.transaction_date}
                    </div>
                </div>
                <div className="row align-items-center mb-3">
                    <div className="col-auto">
                        <label htmlFor="name" className="col-form-label">
                            Name
                        </label>
                    </div>
                    <div className="col-auto">
                        {transactionModel && transactionModel.name}
                    </div>
                </div>
            </div>
        )

}

export default TransactionViewDetail;