use rocket::{catchers, launch, routes};
use rocket_okapi::{openapi_get_routes, rapidoc::*, swagger_ui::*, OpenApiError};

// use dotenv;
// use std::env;

mod catchers;
mod config;
mod consts;
mod models;
mod routes;
mod schema;
mod utils;

/// Main function to start the server
#[launch]
fn rocket() -> _ {
    // dotenv::dotenv().expect("Failed to read .env file");

    // // let app_host = env::var("APP_HOST").expect("APP_HOST must be set");
    // // let app_port = env::var("APP_PORT").expect("APP_PORT must be set");
    // let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let db_pool = config::db::establish_connection(&db_url);

    rocket::build()
        .manage(config::db::establish_connection(
            "postgres://postgres:postgres@localhost:5432/postgres",
        ))
        .mount(
            "/",
            openapi_get_routes![
                routes::todos::get_todos,
                routes::todos::new_todo,
                routes::todos::update_todo,
                routes::user::signup,
                routes::user::login,
                routes::user::restricted
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .register(
            "/",
            catchers![
                catchers::not_found,
                catchers::internal_server_error,
                catchers::unauthorized
            ],
        )
}
