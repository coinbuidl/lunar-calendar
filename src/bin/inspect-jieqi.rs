use chinese_lunisolar_calendar::{LunisolarDate, SolarDate, SolarYear};

fn main() {
    let year = 2026;
    let sy = SolarYear::from_u16(year);
    // According to some versions, SolarYear has a solar_terms method
    // let terms = sy.solar_terms(); 
    println!("Solar Year: {:?}", sy);
    
    let sd = SolarDate::from_ymd(year, 2, 4).unwrap();
    let ld = LunisolarDate::from_solar_date(sd).unwrap();
    println!("Lunisolar Date for 2026-02-04: {:?}", ld);
    // Many crates store the term index or similar.
}
