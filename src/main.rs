#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rust_webapp_playground;
extern crate rocket;

use std::env;

#[get("/")]
fn info() -> &'static str {
    "Hello"
}

fn main() {
    let db_url = match env::var("DATABASE_URL") {
        Ok(url)  => url,
        Err(_) => {
            eprintln!("DATABASE_URLが環境変数に定義されていません");
            std::process::exit(1);
        }
    };
    rocket::ignite()
        .manage(rust_webapp_playground::helpers::mysql::init_mysql_pool(&db_url))
        .mount("/", routes![info])
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
        ])
        .launch();
}
