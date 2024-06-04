import { ChangeEventHandler, ReactNode, useState } from "react";

interface GenericSelectInputProps{
    onChange: ChangeEventHandler<HTMLSelectElement>;
    id: string;
    label: string;
    item: any;
    children: ReactNode;
}


function GenericSelectInput(props:GenericSelectInputProps){

    return(
        <div className="row align-items-center mb-3">
            <div className="col-auto">
                <label htmlFor={props.id} className="col-form-label">
                    {props.label}
                </label>
            </div>
            <div className="col-auto">
                <select name="category" id="category" className="form-select" value={props.item} onChange={props.onChange}>
                    <option value={0}>Select</option>
                    {props.children}
                </select>
            </div>
        </div>
    )
}

export default GenericSelectInput;