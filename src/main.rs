mod user;
mod queries;

#[macro_use]
extern crate rocket;


#[launch]
fn rocket() -> _ {
    queries::set_signing_key();
    rocket::build().mount("/", routes![health])
}

#[get("/_health")]
fn health() {}