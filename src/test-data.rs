use chinese_lunisolar_calendar::LunisolarDate;
use chrono::{Datelike, Local, NaiveDate};

fn main() {
    println!("--- Lunar Date Verification ---");
    
    // Check Today
    let now = Local::now();
    let solar_date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap();
    
    if let Some(lunar_date) = LunisolarDate::from_solar_date(solar_date) {
        println!("Solar Date: {}", solar_date);
        println!("Lunar Date: {}", lunar_date.to_string());
        println!("Year Stem-Branch: {:?}", lunar_date.get_year_data().get_stem_branch());
    } else {
        println!("Failed to calculate lunar date.");
    }
}
