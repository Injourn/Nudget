import { FormEventHandler, ReactNode } from "react";

interface GenericFormProps{
    children: ReactNode;
    onSubmit: FormEventHandler<HTMLFormElement>;
    edit?: boolean;
    cancellable?:boolean;
}


function GenericForm(props:GenericFormProps){

    return (
        <form onSubmit={props.onSubmit}>
            {props.children}
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <input type="submit" className="btn" data-bs-dismiss="modal" value={props.edit ? "Edit entry" : "Add Entry"} />
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