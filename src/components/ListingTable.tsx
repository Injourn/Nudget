import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import ListingModel from "../models/ListingModel";
import ListingEntry from "./ListingEntry";
import Modal from "./Modal";
import ListingViewDetail from "./ListingViewDetail";
import ListingAddEdit from "./ListingAddEdit";

function ListingTable(){
    const defaultValue: string = getLocalDate();
    const [tableData, setTableData] = useState<ListingModel[]>([]);
    //TODO: Consider removing add state here.
    const [showAdd, setAdd] = useState<boolean>(false);
    const [modalData, setModalData] = useState<ListingModel>({} as ListingModel);

    invoke<ListingModel[]>("get_listing").then(listings => setTableData(listings))

    function toggleAddButton(event: React.MouseEvent<HTMLElement, MouseEvent>): void {
        setAdd(!showAdd);
    }

    function getLocalDate() : string{
        const date:Date = new Date();
        const day : string = date.toLocaleString('en', {day: '2-digit'});   // DD
        const month : string = date.toLocaleString('en', {month: '2-digit'}); // MM
        const year : string = date.toLocaleString('en', {year: 'numeric'}); // YYYY
        return `${year}-${month}-${day}`
    }
    // function onChangeModel(item: any){
    //     setModalData(model => item)
    // }

    function changeModalData(model:ListingModel){
        setModalData(model);
    }

    return (
        <table className="table table-striped">
            <thead>
                <th scope="col">Amount</th>
                <th scope="col">Category</th>
                <th scope="col">Date</th>
                <th scope="col">Name</th>
                <th scope="col">
                        <button type="button" className="btn btn-primary"
                         data-bs-toggle="modal" data-bs-target="#listingModalAddEdit"
                         onClick={() => changeModalData({date: defaultValue } as ListingModel)}>
                            +
                        </button>
                </th>
            </thead>
            <tbody>
                {showAdd && <ListingEntry viewType="edit" />}
            {tableData.map((data,i) => 
                <tr>
                    <td>{data.amount}</td>
                    <td>{data.category}</td>
                    <td>{data.date}</td>
                    <td>{data.name}</td>
                    <td>
                        <button type="button" className="btn btn-primary"
                        data-bs-toggle="modal" data-bs-target="#listingModalView"
                        onClick={() => changeModalData(data)}>
                            {i}
                        </button>
                        <button type="button" className="btn btn-primary"
                        data-bs-toggle="modal" data-bs-target="#listingModalAddEdit"
                        onClick={() => changeModalData(data)}>
                            {i}
                        </button>
                    </td>
                </tr>
                )
            }
            <Modal name="listingModalView" title="View Listing">
              <ListingViewDetail entry={modalData} />
            </Modal>
            <Modal name="listingModalAddEdit" title="Add Listing">
                <ListingAddEdit entry={modalData}/>
            </Modal>
            </tbody>
        </table>
)
}

{/* <button type="button" className="btn btn-primary" data-bs-toggle="modal" data-bs-target="#listingModal">
              Launch demo modal
            </button>
            <Modal name="listingModal" title="AddListing">
              <ListingAddEdit />
            </Modal> */}

export default ListingTable;