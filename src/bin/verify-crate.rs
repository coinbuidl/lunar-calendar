use chinese_lunisolar_calendar::{LunisolarDate, SolarDate};
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use std::env;

fn parse_input_date(input: &str) -> Result<NaiveDate, String> {
    if let Ok(d) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        return Ok(d);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M") {
        return Ok(dt.date());
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S") {
        return Ok(dt.date());
    }
    Err(format!(
        "Invalid input '{}'. Use YYYY-MM-DD or YYYY-MM-DD HH:MM[:SS].",
        input
    ))
}

fn main() {
    println!("--- Lunar Date Verification ---");

    let date = if let Some(input) = env::args().nth(1) {
        match parse_input_date(&input) {
            Ok(d) => d,
            Err(msg) => {
                eprintln!("{msg}");
                std::process::exit(2);
            }
        }
    } else {
        let now = Local::now();
        match NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()) {
            Some(d) => d,
            None => {
                eprintln!("Failed to construct local date from system clock.");
                std::process::exit(2);
            }
        }
    };

    let solar_date =
        match SolarDate::from_ymd(date.year() as u16, date.month() as u8, date.day() as u8) {
            Ok(sd) => sd,
            Err(e) => {
                eprintln!("Failed to build SolarDate: {:?}", e);
                std::process::exit(2);
            }
        };

    match LunisolarDate::from_solar_date(solar_date) {
        Ok(lunar_date) => {
            println!("Solar Date: {}", solar_date);
            println!("Lunar Date: {}", lunar_date.to_string());
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
