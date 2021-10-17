use rocket::launch;
use csvion::rocket_builder;

#[launch]
fn rocket() -> _ {
    rocket_builder()
}

