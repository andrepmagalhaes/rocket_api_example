use rocket::serde::json::Json;
use rocket::{get, post, put, State};
use rocket_okapi::openapi;
use serde::{Deserialize, Serialize};

use crate::config::db::{get_connection, PoolConnection};
use crate::consts::Response;
use crate::models::todos::{Todo, TodoDTO};
use crate::utils::jwt::TokenValidation;

/// Route to get all todos from a user
///
/// # Arguments
///
/// * `_dbpool` - A pool of database connections
/// * `_token_validation` - A struct containing the token validation result
///
/// # Returns
///
/// * A Json containing the response - for reference, see `Response` struct in `consts.rs`
#[openapi(tag = "Todo")]
#[get("/todos", format = "application/json")]
pub fn get_todos(
    _dbpool: &State<PoolConnection>,
    _token_validation: TokenValidation,
) -> Json<Response<Todo>> {
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

    let todos_result = Todo::get_todos(_token_validation.claims.sub, &mut db_connection);

    match todos_result {
        Ok(todos) => {
            return Json(Response {
                message: "Todos fetched successfully".to_string(),
                data: todos,
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

/// Route to create a new todo
///
/// # Arguments
///
/// * `new_todo` - A Json containing the new todo details. For reference, see `TodoDTO` struct in `models/todos.rs`
/// * `_dbpool` - A pool of database connections
/// * `_token_validation` - A struct containing the token validation result
///
/// # Returns
///
/// * A Json containing the response - for reference, see `Response` struct in `consts.rs`
#[openapi(tag = "Todo")]
#[post("/todo", format = "application/json", data = "<new_todo>")]
pub fn new_todo(
    new_todo: Json<TodoDTO>,
    _dbpool: &State<PoolConnection>,
    _token_validation: TokenValidation,
) -> Json<Response<i8>> {
    match validate_input(new_todo.clone().into_inner()) {
        Ok(_) => {}
        Err(message) => {
            return Json(Response {
                message: message,
                data: vec![],
            });
        }
    }

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

    let todo = TodoDTO {
        user_id: Some(_token_validation.claims.sub),
        title: new_todo.title.clone(),
        description: new_todo.description.clone(),
        completed: Some(false),
    };

    let new_todo_result = Todo::new_todo(todo, &mut db_connection);

    match new_todo_result {
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

/// Route to update a todo
///
/// # Arguments
///
/// * `todo_id` - The id of the todo to be updated
/// * `update_todo` - A Json containing the updated todo details. For reference, see `TodoDTO` struct in `models/todos.rs`
/// * `_dbpool` - A pool of database connections
/// * `_token_validation` - A struct containing the token validation result
///
/// # Returns
///
/// * A Json containing the response - for reference, see `Response` struct in `consts.rs`
#[openapi(tag = "Todo")]
#[post("/todo/<todo_id>", format = "application/json", data = "<update_todo>")]
pub fn update_todo(
    todo_id: i32,
    update_todo: Json<TodoDTO>,
    _dbpool: &State<PoolConnection>,
    _token_validation: TokenValidation,
) -> Json<Response<i8>> {
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

    let todo = TodoDTO {
        user_id: Some(_token_validation.claims.sub),
        title: update_todo.title.clone(),
        description: update_todo.description.clone(),
        completed: update_todo.completed.clone(),
    };

    let update_todo_result = Todo::update_todo(todo_id, todo, &mut db_connection);

    match update_todo_result {
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

/// Internal function to validate the input on creating a new todo
///
/// # Arguments
///
/// * `todo` - A struct containing the todo details. For reference, see `TodoDTO` struct in `models/todos.rs`
///
/// # Returns
///
/// * An empty result if the input is valid, otherwise an error message
fn validate_input(todo: TodoDTO) -> Result<(), String> {
    if todo.user_id.is_some() {
        return Err("user_id cannot be a parameter".to_string());
    }

    if todo.completed.is_some() {
        return Err("completed cannot be a parameter".to_string());
    }

    if todo.title.is_none() {
        return Err("paramter title is required".to_string());
    }

    if todo.description.is_none() {
        return Err("paramter description is required".to_string());
    }

    Ok(())
}
