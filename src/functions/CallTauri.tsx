import { invoke } from "@tauri-apps/api";
import TauriResponse from "../models/TauriResponse";
import { InvokeArgs } from "@tauri-apps/api/tauri";
import { toast } from "react-toastify";

async function callTauri<T>(commandName:string,args?: InvokeArgs){
    let response: TauriResponse<T> = await invoke<TauriResponse<T>>(commandName,args);

    if(response.success){
        return response.response;
    } else {
        console.log("From: " + commandName + "\nError:" + response.error_msg);
        toast.error("Error: "  + response.error_msg);
        throw new Error(response.error_msg);
    }
}

export default callTauri;