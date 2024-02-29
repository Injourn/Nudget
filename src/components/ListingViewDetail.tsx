import ListingModel from "../models/ListingModel";


function ListingViewDetail(props:any){
    const listingModel:ListingModel = props.entry;


    return (
        <div>
            <div className="row align-items-center mb-3">
                    <div className="col-auto">
                        <label htmlFor="amount" className="col-form-label">
                            Amount
                        </label>
                    </div>
                    <div className="col-auto">
                        {listingModel && listingModel.amount}
                    </div>
                </div>
                <div className="row align-items-center mb-3">
                    <div className="col-auto">
                        <label htmlFor="catgory" className="col-form-label">
                            Category
                        </label>
                    </div>
                    <div className="col-auto">
                        {listingModel && listingModel.category}
                    </div>
                </div>
                <div className="row align-items-center mb-3">
                    <div className="col-auto">
                        <label htmlFor="date" className="col-form-label">
                            Date
                        </label>
                    </div>
                    <div className="col-auto">
                        {listingModel && listingModel.date}
                    </div>
                </div>
                <div className="row align-items-center mb-3">
                    <div className="col-auto">
                        <label htmlFor="name" className="col-form-label">
                            Name
                        </label>
                    </div>
                    <div className="col-auto">
                        {listingModel && listingModel.name}
                    </div>
                </div>
            </div>
        )

}

export default ListingViewDetail;