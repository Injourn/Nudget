import { FormEventHandler, MouseEventHandler, ReactNode, useState } from "react";
import Modal from "../ui/Modal";

interface GenericFormProps{
    onRemove: MouseEventHandler<HTMLButtonElement> | undefined;
    children: ReactNode;
    onSubmit: FormEventHandler<HTMLFormElement>;
    edit?: boolean;
    cancellable?:boolean;
    
}


function GenericForm(props:GenericFormProps){
    const [areYouSure,setAreYouSure] = useState<boolean>(false);

    return (
        <form onSubmit={props.onSubmit}>
            {props.children}
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <input type="submit" className="btn" data-bs-dismiss="modal" value={props.edit ? "Edit entry" : "Add Entry"} />
                    { areYouSure ?
                        <button type="button" className="btn-secondary" data-bs-dismiss="modal" aria-label="Close" onClick={props.onRemove}>
                            Are You Sure?
                        </button>
                        :
                        <button type="button" onClick={() => setAreYouSure(true)}>Delete</button>
                    }
                    {props.cancellable ? 
                        <button type="button" className="btn-secondary" data-bs-dismiss="modal" aria-label="Close">Cancel</button> 
                        : null
                    }
                    
                </div>
            </div>
        </form>
    )
}

export default GenericForm;