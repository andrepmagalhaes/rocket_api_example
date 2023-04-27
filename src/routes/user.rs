use rocket::serde::json::Json;
use rocket::{get, post, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::{Deserialize, Serialize};

use crate::config::db::{get_connection, PoolConnection};
use crate::consts::Response;
use crate::models::user::{User, UserDTO, UserLoginDTO};
use crate::utils::jwt::{generate_token, TokenValidation};

/// Struct to hold the response for login
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct LoginResponse {
    /// Token generated for the user for authentication
    token: String,
}

/// Route to signup a new user
///
/// # Arguments
///
/// * `user_signup` - A Json containing the new user details. For reference, see `UserDTO` struct in `models/user.rs`
/// * `_dbpool` - A pool of database connections
///
/// # Returns
///
/// * A Json containing the response - for reference, see `Response` struct in `consts.rs`
#[openapi(tag = "User")]
#[post("/signup", format = "application/json", data = "<user_signup>")]
pub fn signup(user_signup: Json<UserDTO>, _dbpool: &State<PoolConnection>) -> Json<Response<i32>> {
    let db_connection_result = get_connection(_dbpool);
    let mut db_connection = match db_connection_result {
        Ok(conn) => conn,
        Err(_) => {
            return Json(Response {
                message: "Failed to get connection".to_string(),
                data: vec![],
            });
        }
    };

    let signup_result = User::signup(user_signup.into_inner(), &mut db_connection);

    match signup_result {
        Ok(message) => {
            return Json(Response {
                message: message,
                data: vec![],
            });
        }
        Err(message) => {
            return Json(Response {
                message: message,
                data: vec![],
            });
        }
    }
}

/// Route to login a user
///
/// # Arguments
///
/// * `user_login` - A Json containing the user login details. For reference, see `UserLoginDTO` struct in `models/user.rs`
/// * `_dbpool` - A pool of database connections
///
/// # Returns
///
/// * A Json containing the response in which has the token under data parameter - for reference, see `Response` struct in `consts.rs`
#[openapi(tag = "User")]
#[post("/login", format = "application/json", data = "<user_login>")]
pub fn login(
    user_login: Json<UserLoginDTO>,
    _dbpool: &State<PoolConnection>,
) -> Json<Response<LoginResponse>> {
    let db_connection_result = get_connection(_dbpool);
    let mut db_connection = match db_connection_result {
        Ok(conn) => conn,
        Err(_) => {
            return Json(Response {
                message: "Failed to get connection".to_string(),
                data: vec![],
            });
        }
    };

    let fetch_user = User::login(user_login.into_inner(), &mut db_connection);

    let user_data = match fetch_user {
        Ok(data) => data,
        Err(message) => {
            return Json(Response {
                message: message,
                data: vec![],
            });
        }
    };

    let token = generate_token(user_data);

    match token {
        Ok(token) => {
            let l = LoginResponse { token: token };
            return Json(Response {
                message: "Login successful".to_string(),
                data: vec![l],
            });
        }
        Err(message) => {
            return Json(Response {
                message: message,
                data: vec![],
            });
        }
    }
}

/// Route to test the restricted route
///
/// # Arguments
///
/// * `_token_validation` - A struct containing the token validation details. For reference, see `TokenValidation` struct in `utils/jwt.rs`
///
/// # Returns
///
/// * A Json containing the response - for reference, see `Response` struct in `consts.rs`
#[openapi(tag = "User")]
#[get("/restricted", format = "application/json")]
pub fn restricted(_token_validation: TokenValidation) -> Json<Response<String>> {
    println!("{:?}", _token_validation);
    return Json(Response {
        message: "Restricted".to_string(),
        data: vec![],
    });
}
