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
            <TransactionTable />
            <CategoryList />
          </div>

        </div>
      </div>
    </>
  );
}

export default App;
