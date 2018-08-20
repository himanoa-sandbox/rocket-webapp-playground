#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate rust_webapp_playground;
extern crate serde;
#[macro_use]
extern crate serde_json;

use rocket_contrib::json::Json;
use std::env;

use diesel::RunQueryDsl;
use rust_webapp_playground::helpers::mysql::{init_mysql_pool, DbConn};
use rust_webapp_playground::models::user::NewUser;
use rust_webapp_playground::schema::users::dsl::users;

#[get("/")]
fn info() -> &'static str {
    "Hello"
}

#[post("/user", data = "<new_user>", format = "application/json")]
pub fn create_user(new_user: Json<NewUser>, db: DbConn) -> rocket_contrib::Json {
    let new_users = vec![NewUser {
        name: new_user.0.name,
    }];
    let result = diesel::insert_into(users).values(&new_users).execute(&*db);
    match result {
        Ok(_) => Json(json!({
            "status": "created"
        })),
        Err(_) => Json(json!({
            "status": "err"
        })),
    }
}

fn main() {
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("DATABASE_URLが環境変数に定義されていません");
            std::process::exit(1);
        }
    };
    rocket::ignite()
        .manage(init_mysql_pool(&db_url))
        .mount("/", routes![info])
        .mount("/", routes![create_user])
        .catch(catchers![
            rust_webapp_playground::error_handlers::bad_request,
            rust_webapp_playground::error_handlers::unauthorized,
            rust_webapp_playground::error_handlers::payment_required,
            rust_webapp_playground::error_handlers::forbidden,
            rust_webapp_playground::error_handlers::not_found,
            rust_webapp_playground::error_handlers::method_not_allowed,
            rust_webapp_playground::error_handlers::not_acceptable,
            rust_webapp_playground::error_handlers::proxy_authentication_required,
            rust_webapp_playground::error_handlers::request_timeout,
            rust_webapp_playground::error_handlers::confrict,
            rust_webapp_playground::error_handlers::gone,
            rust_webapp_playground::error_handlers::length_required,
            rust_webapp_playground::error_handlers::procondition_failed,
            rust_webapp_playground::error_handlers::payload_too_large,
            rust_webapp_playground::error_handlers::internal_error,
            rust_webapp_playground::error_handlers::not_implemented,
            rust_webapp_playground::error_handlers::bad_gateway,
            rust_webapp_playground::error_handlers::service_unavailable,
            rust_webapp_playground::error_handlers::gateway_timeout
        ]).launch();
}
