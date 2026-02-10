use chrono::{Datelike, NaiveDate, NaiveDateTime};
use lunar_calendar::{
    MAX_SUPPORTED_YEAR, MIN_SUPPORTED_YEAR, get_jieqi_context, get_pillars, get_year_data,
};
use std::env;

fn parse_input_datetime(input: &str) -> Result<NaiveDateTime, String> {
    if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S") {
        return Ok(dt);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M") {
        return Ok(dt);
    }
    if let Ok(d) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        if let Some(dt) = d.and_hms_opt(0, 0, 0) {
            return Ok(dt);
        }
    }
    Err(format!(
        "Invalid input '{}'. Use YYYY-MM-DD or YYYY-MM-DD HH:MM[:SS].",
        input
    ))
}

fn main() {
    let mut input: Option<String> = None;
    let mut verify_with_crate = false;
    for arg in env::args().skip(1) {
        if arg == "--verify-with-crate" {
            verify_with_crate = true;
        } else if input.is_none() {
            input = Some(arg);
        } else {
            eprintln!(
                "Too many arguments. Usage: query-lunisolar [YYYY-MM-DD|YYYY-MM-DD HH:MM[:SS]] [--verify-with-crate]"
            );
            std::process::exit(2);
        }
    }

    let input = input.unwrap_or_else(|| "2026-02-04 00:00:00".to_string());
    let dt = match parse_input_datetime(&input) {
        Ok(d) => d,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(2);
        }
    };

    if !(MIN_SUPPORTED_YEAR..=MAX_SUPPORTED_YEAR).contains(&dt.year()) {
        eprintln!(
            "Year out of supported table range ({}..={}): {}",
            MIN_SUPPORTED_YEAR,
            MAX_SUPPORTED_YEAR,
            dt.year()
        );
        std::process::exit(2);
    }

    println!("Input datetime: {}", dt);
    println!("--- Primary path: local table + local algorithm (offline) ---");

    if let Some(year_info) = get_year_data(dt.year()) {
        println!("Gregorian year: {}", year_info.year);
        println!("Lunar New Year (table): {}", year_info.lunar_new_year);
        println!("LiChun (table): {}", year_info.lichun_time);

        if let Ok(lny) = NaiveDate::parse_from_str(year_info.lunar_new_year, "%Y-%m-%d") {
            let day_offset = (dt.date() - lny).num_days();
            if day_offset >= 0 {
                println!(
                    "Day offset from Lunar New Year (table): +{} day(s)",
                    day_offset
                );
            } else {
                println!(
                    "Day offset from Lunar New Year (table): {} day(s)",
                    day_offset
                );
            }
        }
    }

    if let Some(pillars) = get_pillars(dt) {
        println!("BaZi (local algo): {}", pillars);
    } else {
        println!("BaZi (local algo): unavailable for this input");
    }

    if let Some(ctx) = get_jieqi_context(dt) {
        println!("JieQi context (table): {}", ctx);
    } else {
        println!("JieQi context (table): unavailable for this input");
    }

    if verify_with_crate {
        use chinese_lunisolar_calendar::{LunisolarDate, SolarDate};
        println!("--- Verification path: external crate ---");
        let d = dt.date();
        let sd = match SolarDate::from_ymd(d.year() as u16, d.month() as u8, d.day() as u8) {
            Ok(solar_date) => solar_date,
            Err(e) => {
                println!(
                    "Lunar Date (crate verify) failed to build SolarDate: {:?}",
                    e
                );
                return;
            }
        };
        match LunisolarDate::from_solar_date(sd) {
            Ok(ld) => println!("Lunar Date (crate verify): {}", ld),
            Err(e) => println!("Lunar Date (crate verify) failed: {:?}", e),
        }
    }
}
