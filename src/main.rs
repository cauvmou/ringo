#[macro_use]
extern crate rocket;

use std::fs;
use std::sync::Arc;
use std::time::Instant;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use rocket::State;

#[derive(Debug, Clone)]
struct ApplicationData {
    fields: Arc<Vec<String>>,
    start_time: Instant,
}

impl ApplicationData {
    fn establish_db_connection() -> PgConnection {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        PgConnection::establish(&database_url).expect(format!("Failed to connect to database using: {}", database_url).as_str())
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let fields = fs::read_to_string("/var/ringo/list.txt").expect("Failed to read bingo list!")
        .split("\n").map(|s| s.trim().to_string()).collect::<Vec<String>>();
    let data = ApplicationData {
        fields: Arc::new(fields),
        start_time: Instant::now(),
    };
    rocket::build().manage(data).mount("/", routes![health])
}

#[get("/_health")]
fn health(data: &State<ApplicationData>) {}