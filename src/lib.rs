#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate diesel;
extern crate rocket;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate rocket_contrib;
pub mod error_handlers;
pub mod helpers;
pub mod models;
pub mod schema;
