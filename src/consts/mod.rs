use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T>{
    pub message: String,
    pub data: Vec<Option<T>>,
}