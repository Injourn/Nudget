import { ChangeEventHandler } from "react";

interface GenericFormInputProps{
    onChange: ChangeEventHandler<HTMLInputElement> | undefined;
    id: string;
    label: string;
    item: any;
    type: string;
    pattern?:string;
    numeric?:boolean;
}

function GenericFormInput(props:GenericFormInputProps){

    function onInput(e:any){

        if(props.numeric){
            e.target.value = e.target.value.replace(/[^0-9.]/g, '').replace(/(\..*?)\..*/g, '$1').replace(/^0[^.]/, '0');
        }
    }

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
                 pattern={props.pattern}
                 onInput={onInput}
                 onChange={props.onChange}/>
            </div>
        </div>
    )
}

export default GenericFormInput;