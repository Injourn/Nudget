import { ReactNode } from "react";

interface TableProps{
    removeItemBox?: boolean;
    removeItem?(data: any): import("react").MouseEventHandler<HTMLButtonElement> | undefined;
    tableRow(data: any): ReactNode;
    modalTarget?: string;
    addRow?(): import("react").MouseEventHandler<HTMLButtonElement> | undefined;
    addRowBox?: boolean
    tableData: any[];
    columns:string[];
    modals?(): ReactNode;
    

}

function Table(props:TableProps){

    return (
        <table className="table table-striped">
            <thead>
                {props.columns.map((data) =>
                <th scope="col">{data}</th>
                )}
                {props.addRowBox ? 
                <th scope="col">
                    <button type="button" className="btn btn-primary"
                     data-bs-toggle="modal" data-bs-target={"#" + props.modalTarget}
                     onClick={props.addRow}> +
                    </button>
                </th> : null
                }

            </thead>
            <tbody>
                {props.tableData.map((data) => 
                    <tr>
                        {props.tableRow(data)}
                        {props.removeItemBox ? 
                        <td>
                            <button type="button" className="btn btn-primary"
                            onClick={props.removeItem}>
                                -
                            </button>
                        </td> : <></>
                        }
                    </tr>
                )}
            </tbody>
            {props.modals !== undefined ? props.modals() : null}
        </table>
    )
}

export default Table;