use base64::Engine;
use hmac::{Hmac, Mac};
use rand::RngCore;
use rusqlite::Connection;
use sha2::Sha256;
use crate::queries;

pub const GET_SIGNING_KEY: &'static str = include_str!("./queries/get_signing_key.sql");
pub const SET_SIGNING_KEY: &'static str = include_str!("./queries/set_signing_key.sql");

pub fn get_signing_key() -> Hmac<Sha256> {
    let conn = Connection::open("./db/ringo.sqlite").expect("Failed to open database file!");
    let mut statement = conn.prepare(queries::GET_SIGNING_KEY).expect("Failed to prepare query");
    let string_key = statement.query_map([], |row| {
        Ok(String::from(row.get::<usize, String>(0).expect("No columns returned by db")))
    }).expect("Failed to execute query!")
        .filter(|v| v.is_ok())
        .map(|v| v.unwrap())
        .collect::<Vec<String>>().get(0).expect("No key in DB!")
        .to_owned();
    Hmac::new_from_slice(&base64::engine::general_purpose::STANDARD_NO_PAD.decode(string_key).expect("Failed to decode key!")).expect("Failed to hash key!")
}

pub fn set_signing_key() {
    let conn = Connection::open("./db/ringo.sqlite").expect("Failed to open database file!");
    let mut bytes: [u8; 256] = [0; 256];
    rand::thread_rng().fill_bytes(&mut bytes);
    let key = base64::engine::general_purpose::STANDARD_NO_PAD.encode(bytes);
    conn.execute_batch(queries::SET_SIGNING_KEY.replace(":key", format!("\"{key}\"").as_str()).as_str()).expect("Failed to set key in DB!");
}