import { MouseEventHandler, ReactNode } from "react";

interface DataListProps{
    modalTarget: string;
    addRowClick?: MouseEventHandler<HTMLButtonElement> | undefined;
    listRow(data: any): ReactNode;
    onRowClick?(data: any): void;
    listData: any;
    addRowBox?:boolean;
    

}

function DataList(props:DataListProps){


    return(
        <ul className="list-group list-group-flush">
            {props.addRowBox ? 
                    <li className="list-group-item">
                        <button type="button" className="btn btn-primary"
                        data-bs-toggle="modal" data-bs-target={"#" + props.modalTarget}
                        onClick={props.addRowClick}> Add Entry
                        </button>
                    </li> : null
                }
            {props.listData.map((data: any) =>

                <li data-bs-toggle="modal" data-bs-target={"#" + props.modalTarget} className="list-group-item" onClick={() => props.onRowClick?.(data)}>
                    {props.listRow(data)}
                </li>
            )}
        </ul>
    )
}

export default DataList;