#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

mod routes;
pub mod services;

use rocket::fs::{FileServer, relative};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index, routes::data_routes::csv_data_raw])
    .mount("/data", FileServer::from(relative!("data")))
}