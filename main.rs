// src/main.rs

#[macro_use] extern crate rocket;

mod models;
mod routes;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes::routes())
}