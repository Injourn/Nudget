import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import ListingTable from "./components/ListingTable";
import CategoryList from "./components/CategoryList";
import Navbar from "./components/Navbar";
import Sidebar from "./components/Sidebar";
import ListingAddEdit from "./components/ListingAddEdit";
import Modal from "./components/Modal";

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
            <ListingTable />
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
