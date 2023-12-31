use crate::queries;
use base64::Engine;
use hmac::{Hmac, Mac};
use rand::RngCore;
use rusqlite::Connection;
use sha2::Sha256;

const GET_SIGNING_KEY: &'static str = include_str!("./queries/get_signing_key.sql");
const SET_SIGNING_KEY: &'static str = include_str!("./queries/set_signing_key.sql");
const ROOM_CODE_EXISTS: &'static str = include_str!("./queries/room_code_exists.sql");

pub fn get_signing_key() -> Hmac<Sha256> {
    let conn = Connection::open("./db/ringo.sqlite").expect("Failed to open database file!");
    let mut statement = conn
        .prepare(GET_SIGNING_KEY)
        .expect("Failed to prepare query");
    let string_key = statement
        .query_map([], |row| {
            Ok(row
                .get::<usize, String>(0)
                .expect("No columns returned by db"))
        })
        .expect("Failed to execute query!")
        .filter_map(|v| v.ok())
        .collect::<Vec<String>>()
        .get(0)
        .expect("No key in DB!")
        .to_owned();
    Hmac::new_from_slice(
        &base64::engine::general_purpose::STANDARD_NO_PAD
            .decode(string_key)
            .expect("Failed to decode key!"),
    )
    .expect("Failed to hash key!")
}

pub fn set_signing_key() {
    let conn = Connection::open("./db/ringo.sqlite").expect("Failed to open database file!");
    let mut bytes: [u8; 256] = [0; 256];
    rand::thread_rng().fill_bytes(&mut bytes);
    let key = base64::engine::general_purpose::STANDARD_NO_PAD.encode(bytes);
    conn.execute_batch(
        SET_SIGNING_KEY
            .replace(":key", format!("\"{key}\"").as_str())
            .as_str(),
    )
    .expect("Failed to set key in DB!");
}

pub fn room_code_exists(code: String) -> bool {
    let conn = Connection::open("./db/ringo.sqlite").expect("Failed to open database file!");
    let mut statement = conn
        .prepare(ROOM_CODE_EXISTS)
        .expect("Failed to prepare query");
    let count = *statement
        .query_map(&[("code", format!("'{code}'").as_str())], |row| {
            Ok(row
                .get::<usize, String>(0)
                .expect("No columns returned by db")
                .as_str()
                .parse::<u32>()
                .expect("Failed to parse int!"))
        })
        .expect("Failed to query DB")
        .filter_map(|v| v.ok())
        .collect::<Vec<u32>>()
        .first()
        .unwrap();
    count > 0
}
