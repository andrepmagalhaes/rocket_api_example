use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Default response struct for all endpoints
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    /// Message to be returned
    pub message: String,
    /// Data to be returned which is a vector of a type to be defined by the caller
    pub data: Vec<T>,
}
