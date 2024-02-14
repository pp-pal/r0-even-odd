#![no_main]

use risc0_zkvm::guest::env;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, TimeDelta};

risc0_zkvm::guest::entry!(main);

#[derive(Serialize, Deserialize)]
struct Result {
    is_adult: bool,
}

fn main() {
    // Read input from the host
    let dob_and_today: String = env::read();
    let parts: Vec<&str> = dob_and_today.split("|").collect();
    if parts.len() != 2 {
        panic!("Invalid dob and today input format");
    }

    let dob = parts[0];
    let today = parts[1];

    let dob_date = NaiveDate::parse_from_str(dob, "%Y-%m-%d").expect("Invalid date format");
    let today_date = NaiveDate::parse_from_str(today, "%Y-%m-%d").expect("Invalid date format");
    let age_in_days = today_date.signed_duration_since(dob_date);

    // 18 Years = 6570 days
    let is_adult = age_in_days >= TimeDelta::days(6570);

    let result = Result {
        is_adult,
    };

    // Write public output to the journal.
    // Reference : https://dev.risczero.com/terminology#journal
    // We don't commit the date of birth (DOB); it remains secret.
    env::commit(&result);
}
