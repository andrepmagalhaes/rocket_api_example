use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Identifiable, RunQueryDsl, PgConnection, AsChangeset, prelude::*};
use serde::{Serialize, Deserialize};
use crate::schema::todos::{self, dsl::*};

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug, Clone)]
#[table_name = "todos"]
pub struct TodoDTO {
    pub user_id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

impl Todo {
    pub fn new_todo(data: TodoDTO, conn: &mut PgConnection) -> Result<(), String> {

        let validation = Todo::validate_input_new_todo(data.clone());

        if validation.is_err() {
            return Err(validation.err().unwrap());
        }

        let result = diesel::insert_into(todos)
            .values(&data)
            .execute(conn);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to create todo".to_string()),
        }
    }

    pub fn update_todo(data: TodoDTO, conn: &mut PgConnection) -> Result<(), String> {
        let result = diesel::update(todos)
            .set(&data)
            .execute(conn);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to update todo".to_string()),
        }
    }

    pub fn get_todos(user: i32, conn: &mut PgConnection) -> Result<Vec<Todo>, String> {
        let result = todos
            .filter(user_id.eq(user))
            .load(conn);

        match result {
            Ok(data) => Ok(data),
            Err(_) => Err("Failed to get todos".to_string()),
        }
    }
    
    
    fn validate_input_new_todo(data: TodoDTO) -> Result<(), String> {
        
        if Some(data.title) == None {
            return Err("Title is required".to_string());
        }

        if Some(data.description) == None {
            return Err("Description is required".to_string());
        }

        return Ok(());
    }
}