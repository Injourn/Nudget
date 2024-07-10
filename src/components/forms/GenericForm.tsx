import { FormEventHandler, MouseEventHandler, ReactNode, useState } from "react";

interface GenericFormProps{
    onRemove?: MouseEventHandler<HTMLButtonElement> | undefined;
    children: ReactNode;
    onSubmit: FormEventHandler<HTMLFormElement>;
    edit?: boolean;
    cancellable?:boolean;
    modalName?:string;
    
}


function GenericForm(props:GenericFormProps){
    const [areYouSure,setAreYouSure] = useState<boolean>(false);


    if(props.modalName !== undefined){
        let modal = document.getElementById(props.modalName);
        if(modal != null){
            modal.addEventListener('hidden.bs.modal', function () {
                setAreYouSure(false);
            })
        }
    }

    return (
        <form onSubmit={props.onSubmit}>
            {props.children}
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <input type="submit" className="btn" data-bs-dismiss="modal" value={props.edit ? "Edit entry" : "Add Entry"} onClick={() => setAreYouSure(false)}/>
                    {props.edit ?     
                        areYouSure ?
                            <button type="button" className="btn-secondary" data-bs-dismiss="modal" aria-label="Close" onClick={props.onRemove}>
                                Are You Sure?
                            </button>
                            :
                            <button type="button" onClick={() => setAreYouSure(true)}>Delete</button>
                         : <></>
                    }
                    {props.cancellable ? 
                        <button type="button" className="btn-secondary" data-bs-dismiss="modal" aria-label="Close" onClick={() => setAreYouSure(false)}>Cancel</button> 
                        : null
                    }
                    
                </div>
            </div>
        </form>
    )
}

export default GenericForm;