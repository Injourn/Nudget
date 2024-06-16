import { ChangeEventHandler } from "react";

interface GenericFormInputProps{
    onChange: ChangeEventHandler<HTMLInputElement> | undefined;
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
                <input 
                 type={props.type} 
                 id={props.id} 
                 className={"form-control" + (props.type == "checkbox" && "form-check-input")}
                 value={props.item} 
                 checked={props.item}
                 onChange={props.onChange}/>
            </div>
        </div>
    )
}

export default GenericFormInput;