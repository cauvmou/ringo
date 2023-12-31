mod queries;
mod user;

#[macro_use]
extern crate rocket;

use rand::Rng;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[launch]
fn rocket() -> _ {
    queries::set_signing_key();
    rocket::build().mount("/", routes![health, post_room])
}

#[get("/_health")]
fn health() {}

#[derive(Clone, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct AuthBearer {
    bearer: String,
}

#[post("/room?<username>")]
fn post_room(username: &str, ws: ws::WebSocket) -> Json<AuthBearer> {
    let mut chars: [char; 6] = [char::default(); 6];
    loop {
        rand::thread_rng().fill(&mut chars);
        if !queries::room_code_exists(chars.iter().collect()) {
            break;
        }
    }
    let code: String = chars.iter().collect();
    // 3. Setup DB rows

    // 4. Return user

    Json(AuthBearer {
        bearer: user::User::new(code, username.to_string()).into(),
    })
}
