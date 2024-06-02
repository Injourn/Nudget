import { FormEventHandler, ReactNode } from "react";

interface GenericFormProps{
    children: ReactNode;
    onSubmit: FormEventHandler<HTMLFormElement>;
    edit?: boolean;
}


function GenericForm(props:GenericFormProps){

    return (
        <form onSubmit={props.onSubmit}>
            {props.children}
            <div className="row align-items-center mb-3">
                <div className="col-auto">
                    <input type="submit" className="btn" data-bs-dismiss="modal" value={props.edit ? "Edit entry" : "Add Entry"} />
                </div>
            </div>
        </form>
    )
}

export default GenericForm;