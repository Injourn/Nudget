import { FormEventHandler, ReactNode } from "react";

interface GenericFormProps{
    children: ReactNode;
    onSubmit: FormEventHandler<HTMLFormElement>;
}


function GenericForm(props:GenericFormProps){

    return (
        <form onSubmit={props.onSubmit}>
            {props.children}
        </form>
    )
}

export default GenericForm;