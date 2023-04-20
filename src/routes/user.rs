use rocket::{post, State};
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use crate::models::user::{UserDTO, User, UserLoginDTO};
use crate::config::db::{PoolConnection, get_connection};
use crate::utils::jwt::generate_token;
use crate::consts::Response;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    token: String,
}


#[post("/signup", format = "application/json",  data = "<user_signup>")]
pub fn signup(user_signup: Json<UserDTO>, _dbpool: &State<PoolConnection>) -> Json<Response<i32>> {

    let db_connection_result = get_connection(_dbpool);
    let mut db_connection = match db_connection_result {
        Ok(conn) => conn,
        Err(_) => {
            return Json(Response {
                message: "Failed to get connection".to_string(),
                data: vec![],
            });
        },
    };

    let signup_result = User::signup(user_signup.into_inner(), &mut db_connection);

    match signup_result {
        Ok(message) => {
            return Json(Response {
                message: message,
                data: vec![],
            });
        },
        Err(message) => {
            return Json(Response {
                message: message,
                data: vec![],
            });
        },
    }

}

#[post("/login", format = "application/json",  data = "<user_login>")]
pub fn login(user_login: Json<UserLoginDTO>, _dbpool: &State<PoolConnection>) -> Json<Response<LoginResponse>> {
    
        let db_connection_result = get_connection(_dbpool);
        let mut db_connection = match db_connection_result {
            Ok(conn) => conn,
            Err(_) => {
                return Json(Response {
                    message: "Failed to get connection".to_string(),
                    data: vec![],
                });
            },
        };
    
        let fetch_user = User::login(user_login.into_inner(), &mut db_connection);
    
        let user_data = match fetch_user {
            Ok(data) => data,
            Err(message) => {
                return Json(Response {
                    message: message,
                    data: vec![],
                });
            },
        };

        let token = generate_token(user_data);

        match token {
            Ok(token) => {
                let l = LoginResponse {
                    token: token,
                };
                return Json(Response {
                    message: "Login successful".to_string(),
                    data: vec![Some(l)],
                });
            },
            Err(message) => {
                return Json(Response {
                    message: message,
                    data: vec![],
                });
            },
        }



}