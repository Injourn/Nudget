import { useEffect, useState } from "react";
import BudgetView from "../BudgetView";
import callTauri from "../../functions/CallTauri";
import BudgetPlanModel from "../../models/BudgetPlanModel";
import Cycle from "../../models/Cycle";


function BudgetCalendar(){
    const currentDate:Date = new Date();
    let [activeBudgetPlan,setActiveBudgetPlan] = useState<BudgetPlanModel>({} as BudgetPlanModel);
    let [dateRange,setDateRange] = useState<string>(currentDate.getFullYear() + "-" + (currentDate.getMonth()).toLocaleString('en-US', {minimumIntegerDigits: 2, useGrouping:false}) + "-"
    + (activeBudgetPlan.start_date_of_month ?? currentDate.getDate()));
    let [startDate,setStartDate] = useState<string>();
    let [endDate,setEndDate] = useState<string>();
    useEffect(() => {
        callTauri<BudgetPlanModel>("get_active_budget_plan").then(budgetPlan => setActiveBudgetPlan(budgetPlan));
    },[])


    return(
        <>
            {activeBudgetPlan.cycle === Cycle.MONTHLY ? 
                <MonthlyCycle activeBudgetPlan={activeBudgetPlan} currentDate={currentDate} setDateRange={setDateRange}/> :
            <WeeklyCycle currentDate={currentDate} setDateRange={setDateRange} setStartDate={setStartDate} setEndDate={setEndDate}
                 biWeekly={activeBudgetPlan.cycle === Cycle.BIWEEKLY}/>}
            <BudgetView startDate={startDate} endDate={endDate} budgetDateRange={dateRange}/>
        </>
    );
}

function WeeklyCycle(props:any){
    let daySpread = props.biWeekly ? 14 : 7
    let [startOfWeek,setStartOfWeek] = useState<Date>(props.currentDate);
    let [endOfWeek,setEndOfWeek] = useState<Date>(() => {
        let nextWeek = new Date(props.currentDate)
        nextWeek.setDate(props.currentDate.getDate() + daySpread)
        return nextWeek
    });
    useEffect(() => {
        let nextWeek = new Date(startOfWeek)
        nextWeek.setDate(startOfWeek.getDate() + daySpread)
        setEndOfWeek(nextWeek);
    },[props])
    props.setStartDate(startOfWeek.toISOString().split("T")[0]);
    props.setEndDate(endOfWeek.toISOString().split("T")[0]);
    function addDaysToStartOfWeek(days:number){
        let newDate:Date = startOfWeek;
        newDate.setDate(newDate.getDate() + days);
        let nextWeek:Date = new Date(newDate);
        nextWeek.setDate(newDate.getDate() + daySpread);
        setStartOfWeek(newDate);
        setEndOfWeek(nextWeek);
    }

    return(
        <div className="d-flex">
            <div className="col text-center border rounded-3" onClick={() => addDaysToStartOfWeek(-daySpread)}>&lt;</div>
            <div className="col text-center border rounded-3 bg-primary text-white"><h3>{startOfWeek.toISOString().split("T")[0] + " - " + endOfWeek.toISOString().split("T")[0]}</h3></div>
            <div className="col text-center border rounded-3" onClick={() => addDaysToStartOfWeek(daySpread)}>&gt;</div>
        </div>
    )
}

function MonthlyCycle(props:any){
    let monthsAbbr = ["Jan","Feb","Mar","Apr","May",
        "Jun","Jul","Aug","Sept","Oct","Nov","Dec"];
    let months = ["January","February","March","April","May",
        "June","July","August","September","October","November","December"];
    let selectedMonthNumeric = props.currentDate.getMonth()
    let [selectedMonth,setSelectedMonth] = useState<string>(monthsAbbr[selectedMonthNumeric]);
    let [selectedYear,setSelectedYear] = useState<number>(props.currentDate.getFullYear());;
    let exampleYearList = [2023,2024,2025];
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

    props.setDateRange(selectedYear + "-" + (index + 1).toLocaleString('en-US', {minimumIntegerDigits: 2, useGrouping:false}) + "-"
        + (props.activeBudgetPlan.start_date_of_month ?? props.currentDate.getDate()));

    return (
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
                {monthsAbbr.map((month) => 
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
    )
}

export default BudgetCalendar;