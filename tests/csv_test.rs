#[cfg(test)]
mod csv_test {
    use csvion::rocket_builder;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn check_csv_raw() {
        let client = Client::tracked(rocket_builder()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}