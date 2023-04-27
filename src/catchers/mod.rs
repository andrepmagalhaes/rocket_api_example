use crate::consts::Response;
use rocket::{catch, serde::json::Json};

/// Catcher for 401 Unauthorized responses.
#[catch(401)]
pub fn unauthorized() -> Json<Response<i8>> {
    Json(Response {
        message: "Unauthorized".to_string(),
        data: vec![],
    })
}

/// Catcher for 404 Not Found responses.
#[catch(404)]
pub fn not_found() -> Json<Response<i8>> {
    Json(Response {
        message: "Not found".to_string(),
        data: vec![],
    })
}

/// Catcher for 500 Internal Server Error responses.
#[catch(500)]
pub fn internal_server_error() -> Json<Response<i8>> {
    Json(Response {
        message: "Internal server error".to_string(),
        data: vec![],
    })
}
