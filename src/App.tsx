import {Route, Routes} from 'react-router-dom'
import { ToastContainer } from "react-toastify";
import 'react-toastify/dist/ReactToastify.css';
import { useState } from "react";
import {
  Chart as ChartJS,
  CategoryScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  ArcElement,
} from 'chart.js';
import LoadingPage from "./web/layout/LoadingPage";
import CategoryList from "./web/layout/CategoryList";
import BudgetTable from "./web/layout/BudgetTable";
import BudgetView from "./web/layout/BudgetView";
import BudgetPlanAddEdit from "./web/forms/BudgetPlanAddEdit";
import BudgetPlanTable from "./web/layout/BudgetPlanTable";
import BudgetPlanView from "./web/layout/BudgetPlanView";
import AccountList from "./web/layout/AccountList";
import Navbar from './web/elements/Navbar';
import Sidebar from './web/elements/Sidebar';
import Home from './web/pages/Home';
function App() {
  let [dbLoaded,setDbLoaded] = useState<Boolean>(false);
  ChartJS.register(
    CategoryScale,
    ArcElement,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
  );

  if(!dbLoaded){

    return (
      <LoadingPage setDbLoaded={setDbLoaded}/>
    )
  } else {
    return (
      <>
        <Navbar />
        <ToastContainer/>
        <div className="container-fluid">
          <div className="row">
            <Sidebar />
            <div className="col-lg-9">
              <Routes>
                <Route path="/" element={<Home />}></Route>
                <Route path="/categories" element={<CategoryList/>}/>
                <Route path="/budget" element={<BudgetTable />} />
                <Route path="/budget/:budgetId" element={<BudgetView showTransactions={false}/>}/>
                <Route path="/budgetPlan/" element={<BudgetPlanTable/>}/>
                <Route path="/budgetPlan/new" element={<BudgetPlanAddEdit/>}/>
                <Route path="/budgetPlan/:budgetPlanId" element={<BudgetPlanView/>}/>
                <Route path="/accounts" element={<AccountList/>}/>
              </Routes>
            </div>
          </div>
        </div>
      </>
    );
  }
}

export default App;
