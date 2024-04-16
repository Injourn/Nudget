
interface GenericFormInputProps{
    id: string;
    label: string;
    item: any;
    type: string;
}


function GenericFormInput(props:GenericFormInputProps){

    return(
        <div className="row align-items-center mb-3">
            <div className="col-auto">
                <label htmlFor={props.id} className="col-form-label">
                    {props.label}
                </label>
            </div>
            <div className="col-auto">
                <input type={props.type} id={props.id} className="form-control" value={props.item} onChange={(e) => props.item = e.target.value}/>
            </div>
        </div>
    )
}

export default GenericFormInput;