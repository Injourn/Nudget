import Navbar from "./components/ui/Navbar";
import Sidebar from "./components/ui/Sidebar";
import {Route, Routes} from 'react-router-dom'
import Home from "./pages/Home"
import CategoryList from "./components/CategoryList";
import BudgetView from "./components/BudgetView";

function App() {
  

  return (
    <>
      <Navbar />
      <div className="container-fluid">
        <div className="row">
          <Sidebar />
          <div className="col-lg-9">
            <Routes>
              <Route path="/" element={<Home />}></Route>
              <Route path="/categories" element={<CategoryList/>}/>
              <Route path="/budget/:budgetId" element={<BudgetView/>}/>
            </Routes>
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
