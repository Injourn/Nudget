import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import TransactionTable from "./components/TransactionTable";
import CategoryList from "./components/CategoryList";
import Navbar from "./components/ui/Navbar";
import Sidebar from "./components/ui/Sidebar";
import TransactionAddEdit from "./components/TransactionAddEdit";
import Modal from "./components/ui/Modal";
import Plan from "./components/Plan";
import BudgetStatisticsView from "./components/BudgetStatisticsView";
import BudgetModel from "./models/BudgetModel";
import Cycle from "./models/Cycle";
import BudgetView from "./components/BudgetView";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <>
      <Navbar />
      <div className="container-fluid">
        <div className="row">
          <Sidebar />
          <div className="col-lg-9">
            <Plan />
            <BudgetView/>
            <TransactionTable />
            <CategoryList />
          </div>

        </div>
      </div>
    </>
  );
}

export default App;
