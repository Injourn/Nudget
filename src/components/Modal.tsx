import { ReactNode } from "react";

interface ModalProps{
    name: string; //identification name
    title: string;
    children: ReactNode;
}


function Modal(props:ModalProps){

    return (
    <div className="modal fade" id={props.name} data-bs-backdrop="static" data-bs-keyboard="false" aria-labelledby={props.name + "Label"} aria-hidden="true">
        <div className="modal-dialog">
            <div className="modal-content">
                <div className="modal-header">
                    <h1 className="modal-title fs-5" id={props.name + "Label"}>{props.title}</h1>
                    <button type="button" className="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>
                <div className="modal-body">
                    {props.children}
                </div>
            </div>
        </div>
    </div>
    )
}

export default Modal;