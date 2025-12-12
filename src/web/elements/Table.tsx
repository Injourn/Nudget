import { ReactNode } from "react";

interface TableProps{
    name?:string;
    tableRow(data: any): ReactNode;
    modalTarget?: string;
    onRowClick?(data: any): void;
    addRowClick?(): import("react").MouseEventHandler<HTMLButtonElement> | undefined;
    addRowBox?: boolean;
    tableData: any[];
    columns:string[];
    modals?(): ReactNode;
    editable?:boolean;

}

function Table(props:TableProps){

    return (
        <table className="table table-striped">
            <thead>
                {props.columns.map((data) =>
                <th scope="col">{data}</th>
                )}
            </thead>
            <tbody>
                {props.addRowBox ? 
                    <tr>
                        <button type="button" className="btn btn-primary"
                        data-bs-toggle="modal" data-bs-target={"#" + props.modalTarget}
                        onClick={props.addRowClick}> Add Entry
                        </button>
                    </tr> : null
                }

                { Array.isArray(props.tableData) && props.tableData.map((data) => 
                    props.editable ? 
                    <tr  data-bs-toggle="modal" data-bs-target={"#" + props.modalTarget} onClick={() => props.onRowClick?.(data)}>
                        {props.tableRow(data)}
                    </tr> 
                    :
                    <tr onClick={() => props.onRowClick?.(data)} >
                        {props.tableRow(data)}
                    </tr> 
                    
                )}
            </tbody>
            {props.modals !== undefined ? props.modals() : null}
        </table>
    )
}

export default Table;