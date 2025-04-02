#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;
use chrono::{DateTime, Timelike, Utc};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use rocket::response::status::NotFound;
use rocket::serde::Serialize;
use rocket::State;

const SEED_BASE: u64 = 1456456;

#[get("/")]
fn index(words: &State<Vec<String>>) -> Result<Json<WordResponse>, NotFound<&str>> {
    let now = SystemTime::now();
    let date_time: DateTime<Utc> = now.into();
    let modified = date_time.with_hour(0)
        .and_then(|dt| dt.with_minute(0))
        .and_then(|dt| dt.with_second(0))
        .and_then(|dt| dt.with_nanosecond(0))
        .expect("whoops");
    let then = modified.timestamp_millis();
    let (seed_result, _) = SEED_BASE.overflowing_mul(then as u64);
    let r = StdRng::seed_from_u64(seed_result).random_range(0..words.len());
    words.get(r).map(|word| Json(WordResponse {
        word: word.clone(),
        timestamp: then,
    })).ok_or_else(|| NotFound("Out of bounds"))
}

#[launch]
fn rocket() -> _ {
    let file = File::open("words.txt").expect("Cannot open words.txt");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    rocket::build().manage(lines).mount("/", routes![index])
}

#[derive(Serialize)]
struct WordResponse {
    word: String,
    timestamp: i64,
}