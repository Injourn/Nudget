interface TauriResponse<T>{
    success:boolean;
    error_msg?:string;
    response:T;
}

export default TauriResponse;