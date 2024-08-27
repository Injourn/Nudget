import Navbar from "./components/ui/Navbar";
import Sidebar from "./components/ui/Sidebar";
import {Route, Routes} from 'react-router-dom'
import Home from "./pages/Home"
import CategoryList from "./components/CategoryList";
import BudgetView from "./components/BudgetView";
import BudgetTable from "./components/BudgetTable";
import BudgetPlanTable from "./components/BudgetPlanTable";
import BudgetPlanView from "./components/BudgetPlanView";
import BudgetPlanAddEdit from "./components/BudgetPlanAddEdit";
import { ToastContainer } from "react-toastify";
import 'react-toastify/dist/ReactToastify.css';
import LoadingPage from "./components/LoadingPage";
import { useState } from "react";

function App() {
  let [dbLoaded,setDbLoaded] = useState<Boolean>(false);

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
