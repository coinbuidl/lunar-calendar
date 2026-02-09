use chinese_lunisolar_calendar::{LunisolarDate, SolarDate};
use chrono::{Datelike, Local};

fn main() {
    println!("--- Lunar Date Verification ---");
    
    let now = Local::now();
    // Convert to the crate's SolarDate type
    let solar_date = SolarDate::from_ymd(now.year() as u16, now.month() as u8, now.day() as u8).unwrap();
    
    match LunisolarDate::from_solar_date(solar_date) {
        Ok(lunar_date) => {
            println!("Solar Date: {}", solar_date);
            println!("Lunar Date: {}", lunar_date.to_string());
        },
        Err(e) => println!("Error: {:?}", e),
    }
}
