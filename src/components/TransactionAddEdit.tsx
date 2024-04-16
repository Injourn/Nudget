import TransactionModel from "../models/TransactionModel";


function TransactionAddEdit(props: any){
    const item: TransactionModel = props.entry;

    function addTransaction(formData:React.SyntheticEvent){
        formData.preventDefault();
    }
    
    return(
        <form onSubmit={addTransaction}>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <label htmlFor="amount" className="col-form-label">
                        Amount
                    </label>
                </div>
                <div className="col-auto">
                    <input type="text" id="amount" className="form-control" value={item.amount} onChange={(e) => item.amount = e.target.value}/>
                </div>
            </div>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <label htmlFor="catgory" className="col-form-label">
                        Category
                    </label>
                </div>
                <div className="col-auto">
                    <select name="category" id="category" className="form-select" value={item.category} onChange={(e) => item.category = e.target.value}>
                        <option value="Rent">Rent</option>
                        <option value="Groceries">Groceries</option>
                    </select>
                </div>
            </div>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <label htmlFor="date" className="col-form-label">
                        Date
                    </label>
                </div>
                <div className="col-auto">
                    <input type="date" id="date" className="form-control" value={item.date} onChange={(e) => item.date = e.target.value}/>
                </div>
            </div>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <label htmlFor="name" className="col-form-label">
                        Name
                    </label>
                </div>
                <div className="col-auto">
                    <input type="name" id="text" className="form-control" value={item.name} onChange={(e) => item.name = e.target.value}/>
                </div>
            </div>
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <input type="submit" className="btn" data-bs-dismiss="modal" value="Add Entry" />
                </div>
            </div>
        </form>
    )
}

export default TransactionAddEdit;