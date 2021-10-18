#[cfg(test)]
mod test {
    use csvion::rocket_builder;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn ping_test() {
        let client = Client::tracked(rocket_builder()).expect("valid rocket instance");
        let response = client.get("/ping").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("PONG!".to_string()));
    }
}