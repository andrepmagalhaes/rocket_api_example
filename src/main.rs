use rocket::{launch, routes};
// use dotenv;
// use std::env;

mod config;
mod schema;
mod models;
mod utils;
mod routes;
mod consts;

#[launch]
fn rocket() -> _ {
    // print!("asdfasdf");
    // dotenv::dotenv().expect("Failed to read .env file");


    // // let app_host = env::var("APP_HOST").expect("APP_HOST must be set");
    // // let app_port = env::var("APP_PORT").expect("APP_PORT must be set");
    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let db_pool = config::db::establish_connection(&db_url);

    print!("building");

    rocket::build()
        .manage(config::db::establish_connection("postgres://postgres:postgres@localhost:5432/postgres"))
        .mount("/", routes![routes::user::signup])
        .mount("/", routes![routes::user::login])

}
