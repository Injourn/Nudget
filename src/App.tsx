import Navbar from "./components/uiElements/Navbar";
import Sidebar from "./components/uiElements/Sidebar";
import {Route, Routes} from 'react-router-dom'
import Home from "./pages/Home"
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
import LoadingPage from "./components/views/LoadingPage";
import CategoryList from "./components/views/CategoryList";
import BudgetTable from "./components/views/BudgetTable";
import BudgetView from "./components/views/BudgetView";
import BudgetPlanAddEdit from "./components/forms/BudgetPlanAddEdit";
import BudgetPlanTable from "./components/views/BudgetPlanTable";
import BudgetPlanView from "./components/views/BudgetPlanView";
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
              </Routes>
            </div>
          </div>
        </div>
      </>
    );
  }
}

export default App;
