use anyhow::Error;

#[derive(serde::Serialize,serde::Deserialize)]
pub(crate) struct Response<T> {
    pub(crate) success:bool,
    pub(crate) error_msg:Option<String>,
    pub(crate) response: Option<T>,
}

impl<T> Response<T> {
    pub fn success(obj:T) -> Self{
        Self {
            success:true,
            error_msg:None,
            response:Some(obj),
        }
    }
    pub fn error(error: Error) -> Self{
        Self { 
            success: false,
            error_msg: Some(error.to_string()), 
            response: None 
        }
    }
}