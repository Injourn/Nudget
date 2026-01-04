import { invoke } from "@tauri-apps/api/core";
import TauriResponse from "../models/response/TauriResponse";
import { InvokeArgs } from "@tauri-apps/api/core";
import { toast } from "react-toastify";

async function callTauri<T>(commandName:string,args?: InvokeArgs){
    console.info("running command " + commandName)
    let response: TauriResponse<T> = await invoke<TauriResponse<T>>(commandName,args);

    if(response.success){
        return response.response;
    } else {
        console.info("From: " + commandName + "\nError:" + response.error_msg);
        toast.error("Error: "  + response.error_msg);
        throw new Error(response.error_msg);
    }
}

export default callTauri;