#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

mod routes;
mod services;

use rocket::{Rocket, Build};
use rocket::fs::{FileServer, relative};

#[get("/")]
fn index() -> &'static str {
    "CSVion"
}

pub fn rocket_builder() -> Rocket<Build> {
    rocket::build()
    .mount("/", routes![index, routes::ping::ping])
    .mount("/api/csv", routes![routes::csv::csv_data_raw])
    .mount("/data", FileServer::from(relative!("data")))
}
