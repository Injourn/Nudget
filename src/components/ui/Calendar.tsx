import { useState } from "react";


function Calendar(props:any){
    let [selectedMonth,setSelectedMonth] = useState<string>("Jul")
    let months = ["Jan","Feb","Mar","Apr","May",
        "Jun","Jul","Aug","Sept","Oct","Nov","Dec"];
    let index = months.indexOf(selectedMonth);


    return(
        <>
            <div className="row">
                <div className="text-center">
                    <h2>
                        <div className="border rounded-2 strong d-inline md-2 px-1">&lt;</div>
                        July 2024 
                        <div className="border rounded-2 strong d-inline md-2 px-1">&gt;</div>
                    </h2>
                </div>
            </div>
            <div className="d-flex d-md-none">
                {index - 1 > 0 && <div className="col text-center border rounded-3" onClick={() => setSelectedMonth(months[index-1])}>{months[index-1]}</div>}
                <div className="col text-center border rounded-3 bg-primary text-white">{selectedMonth}</div>
                {index + 1 < 12 && <div className="col text-center border rounded-3" onClick={() => setSelectedMonth(months[index+1])}>{months[index+1]}</div>}
            </div>
            <div className="row align-items-center d-none d-md-flex">
                {months.map((month,i) => 
                    <>
                        {
                            month === selectedMonth ?
                            <div className="col text-center border rounded-3 bg-primary text-white">{month}</div> :
                            <div className="col text-center border rounded-3" onClick={() => setSelectedMonth(month)}>{month}</div>
                        }
                    </>
                )}
            </div>
        </>
    );
}

export default Calendar;