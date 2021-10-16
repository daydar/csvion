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

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn check_csv_raw() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}