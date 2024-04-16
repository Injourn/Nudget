import TransactionModel from "../models/TransactionModel"


function TransactionEntry(props: any){

    if(props.viewType === "edit"){
        return (
            <Edit entry={props.entry} />
        )
    }
    else {
        return (
            <View entry={props.entry}/>
        )
    }
    

}


function Edit(props :any){

    return(
        <tr>
            <td>
                <input type="text" className="form-control"></input>
            </td>
            <td>
                <select className="form-select">
                    <option>Rent</option>
                    <option>Groceries</option>
                </select>
            </td>
            <td>
                <input type="date" className="form-control"></input>
            </td>
            <td>
                <input type="text" className="form-control"></input>
            </td>
            <td>
                <input type="submit" className="form-control"></input>
            </td>
        </tr>
    )

}

function View(props: any){

    return (
        <tr>
            <td>{props.entry.amount}</td>
            <td>{props.entry.category}</td>
            <td>{props.entry.date}</td>
            <td>{props.entry.name}</td>
            <td>Edit</td>
        </tr>
    )
}

export default TransactionEntry;