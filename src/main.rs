#[macro_use]
extern crate rocket;

use rusqlite::Connection;

const CREATE_TABLES: &'static str = include_str!("create.sql");

#[launch]
fn rocket() -> _ {
    let conn = Connection::open_in_memory().expect("Failed to open in memory DB!");
    conn.execute_batch(CREATE_TABLES).expect("Failed to create tables!");
    rocket::build().mount("/", routes![health])
}

#[get("/_health")]
fn health() {}