import { useEffect, useState } from "react";
import BudgetView from "../BudgetView";
import callTauri from "../../functions/CallTauri";
import BudgetPlanModel from "../../models/BudgetPlanModel";


function BudgetCalendar(props:any){
    const currentDate:Date = new Date();
    let monthsAbbr = ["Jan","Feb","Mar","Apr","May",
        "Jun","Jul","Aug","Sept","Oct","Nov","Dec"];
    let months = ["January","February","March","April","May",
        "June","July","August","September","October","November","December"];
    let selectedMonthNumeric = currentDate.getMonth()
    let [selectedMonth,setSelectedMonth] = useState<string>(monthsAbbr[selectedMonthNumeric]);
    let [selectedYear,setSelectedYear] = useState<number>(currentDate.getFullYear());;
    let exampleYearList = [2023,2024,2025];
    let [activeBudgetPlan,setActiveBudgetPlan] = useState<BudgetPlanModel>({} as BudgetPlanModel);
    useEffect(() => {
        callTauri<BudgetPlanModel>("get_active_budget_plan").then(budgetPlan => setActiveBudgetPlan(budgetPlan));
    },[])

        
    let index = monthsAbbr.indexOf(selectedMonth);

    function setAdjacentMonth(direction:number){
        index += direction;
        if(index >=11){
            index = 0;
            setSelectedYear(selectedYear + 1);
        } else if(index < 0) {
            index = 11;
            setSelectedYear(selectedYear - 1);
        }
        setSelectedMonth(monthsAbbr[index]);
    }


    return(
        <>
            <div className="row">
                <div className="text-center">
                    <h2>
                        <div className="border rounded-2 strong d-inline md-2 px-1" onClick={() => setAdjacentMonth(-1)}>&lt;</div>
                        <div className="btn-group">
                            <button className="dropdown-toggle" style={{boxShadow:"0 0px 0px rgba(0, 0, 0, 0)",padding:"0px 6px",border:"0px solid"}} type="button" id="dropdownMenuButton1" data-bs-toggle="dropdown" aria-expanded="false">
                                {months[index]} {selectedYear}
                            </button>
                            <ul className="dropdown-menu" aria-labelledby="dropdownMenuButton1">
                                {exampleYearList.map((data) => 
                                    <li><a className="dropdown-item" onClick={() => setSelectedYear(data)}>{data}</a></li>
                                )}
                            </ul>
                        </div>
                        <div className="border rounded-2 strong d-inline md-2 px-1" onClick={() => setAdjacentMonth(1)}>&gt;</div>
                    </h2>
                </div>
            </div>
            <div className="d-flex d-md-none">
                {index - 1 >= 0 && <div className="col text-center border rounded-3" onClick={() => setSelectedMonth(monthsAbbr[index-1])}>{monthsAbbr[index-1]}</div>}
                <div className="col text-center border rounded-3 bg-primary text-white">{selectedMonth}</div>
                {index + 1 < 12 && <div className="col text-center border rounded-3" onClick={() => setSelectedMonth(monthsAbbr[index+1])}>{monthsAbbr[index+1]}</div>}
            </div>
            <div className="row align-items-center d-none d-md-flex">
                {monthsAbbr.map((month,i) => 
                    <>
                        {
                            month === selectedMonth ?
                            <div className="col text-center border rounded-3 bg-primary text-white">{month}</div> :
                            <div className="col text-center border rounded-3" onClick={() => setSelectedMonth(month)}>{month}</div>
                        }
                    </>
                )}
            </div>
            <BudgetView budgetDateRange={selectedYear + "-" + (index + 1).toLocaleString('en-US', {minimumIntegerDigits: 2, useGrouping:false}) + "-" + (activeBudgetPlan.start_date_of_month ?? currentDate.getDate())}/>
        </>
    );
}

export default BudgetCalendar;