

function BorderedWindow(props:any){

    return (
        <div className="border p-2 mb-2 rounded-2 shadow-sm">
            {props.children}
        </div>
    );
}

export default BorderedWindow;