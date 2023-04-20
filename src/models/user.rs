
use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, PgConnection, Identifiable, prelude::*};
use serde::{Serialize, Deserialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::schema::users::{self, dsl::*};
use crate::config::db::Connection;


#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct UserDTO {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginDTO {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn signup(user: UserDTO, conn: &mut Connection) -> Result<String, String> {

        let already_exists = User::find_by_email(user.email.clone(), conn);

        if already_exists.is_ok() {
            return Err("User already exists".to_string());
        }

        let hashed_password = hash(user.password, DEFAULT_COST);
        let hashed_password_result = match hashed_password {
            Ok(hashed_password) => hashed_password,
            Err(_) => {
                return Err("Failed to hash password".to_string())
            },
        };

        let new_user = UserDTO {
            name: user.name,
            password: hashed_password_result,
            email: user.email,
        };

        let result = diesel::insert_into(users)
            .values(&new_user)
            .execute(conn);

        if result.is_err() {
            return Err("Failed to create user".to_string());
        }

        return Ok("User created".to_string());
    }

    pub fn login(user_login: UserLoginDTO, conn: &mut Connection) -> Result<User, String> {

        let user = User::find_by_email(user_login.email, conn);

        let user_result = match user {
            Ok(user) => user,
            Err(_) => {
                return Err("User not found".to_string());
            },
        };
        println!("{:?}", user_login.password);
        println!("{:?}", user_result);
        
        let password_matches = verify(user_login.password, &user_result.password);
        
        print!("{:?}", password_matches);

        if password_matches.is_err() {
            return Err("Failed to verify password".to_string());
        }

        if !password_matches.unwrap() {
            return Err("Password does not match".to_string());
        }

        return Ok(user_result);
    }

    fn find_by_email(requested_email: String, conn: &mut PgConnection) -> Result<User, String> {

        let result = users.filter(email.eq(requested_email)).first(conn);

        println!("find_by_email res => {:?}", result);

        match result {
            Ok(user) => return Ok(user),
            Err(_) => return Err("User not found".to_string()),
        }
            
    }
}