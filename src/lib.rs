#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate diesel;
extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
pub mod helpers;
pub mod error_handlers;
