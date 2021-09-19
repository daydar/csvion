#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

mod routes;
pub mod services;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, routes::data_routes::csv_data])
    .launch();
}