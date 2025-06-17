use chrono::{Datelike, Local};
use std::fmt;

/// The Epoch we use is the molad tohu (Day 1 = 1 Tishrei 1 = 7 September -3760)
/// It is a theoretical time point 1 year before creation.
///

#[derive(Clone, Copy)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}

impl Date {
    pub fn from(year: i32, month: u8, day: u8) -> Date {
        Date{year: year, month: month, day: day}
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>4}-{:0>2}-{:0>2}", self.year, self.month, self.day)
    }
}

/// Get today's local date
pub fn today() -> Date {
    let now = Local::now();
    return Date{
        year: now.year(),
        month: now.month() as u8,
        day: now.day() as u8
    }
}

/// Determine if the Jewish year is a leap year
pub fn is_leap_year(year: i32) -> bool {
    match year % 19 {
        0|3|6|8|11|14|17 => true,
        _ => false
    }
}

/// Calculates the molad of the given year
///
/// returns the number of chalakim since the epoch
pub fn molad(year: i32) -> i64 {
    let parts_month = (29*24+12)*1080+793;
    let parts_year  = 12 * parts_month;
    let parts_lyear = 13 * parts_month;
    let parts_cycle = 12 * parts_year + 7 * parts_lyear;
    let total_cycles  = (year-1).div_euclid(19);
    let year_in_cycle = (year-1).rem_euclid(19);
    let mut molad: i64 = (24+5)*1080+204; // molad tohu
    molad += total_cycles as i64 * parts_cycle as i64;
    for year in 0..year_in_cycle {
        if is_leap_year(year+1) {
            molad += parts_lyear as i64;
        } else {
            molad += parts_year as i64;
        }
    }
    return molad;
}

/// Calculates the molad of the given year
///
/// returns the days since epoch, hours (after 6pm), and parts (out of 1080
/// chalakim)
pub fn molad_components(year: i32) -> (i32, u8, u16) {
    let molad = molad(year);
    return ((molad.div_euclid(1080*24)) as i32,
            (molad.div_euclid(1080).rem_euclid(24)) as u8,
            (molad.rem_euclid(1080)) as u16);
}

/// Print molad in a friendly format
pub fn molad_print(year: i32) {
    let (day, hour, parts) = molad_components(year);
    let day = day % 7;
    let hour = (hour + 18) % 24;
    let minute = parts / 18;
    let parts = parts % 18;

    let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let day_str = days[day as usize];
    println!("Molad {year}: {day_str} {hour:0>2}:{minute:0>2} and {parts:>2} \
              chalakim");
}

/// Calculates what day (since epoch) the year starts
pub fn year_start(year: i32) -> i32 {
    let molad = molad(year);
    let day   = molad.div_euclid(1080*24) as i32;
    let parts = molad.rem_euclid(1080*24);
    let mut rosh = day;
    // first rule: if molad is after noon (18 hours after 6pm), Rosh Hashana is
    // postponed 1 day
    if parts >= 18*1080 {
        rosh += 1;
    }
    // second rule: lo ADU
    if rosh % 7 == 0 || rosh % 7 == 3 || rosh % 7 == 5 {
        rosh += 1;
    }
    // third rule: Ga-Ta-RaD
    if !is_leap_year(year) && day % 7 == 2 && parts >= 9*1080+204 {
        rosh = day+2;
    }
    // fourth rule: Be-TU-TeKaPoT
    if is_leap_year(year-1) && day % 7 == 1 && parts >= 15*1080+589 {
        rosh = day+1;
    }
    return rosh;
}

// Calculates the length of the Jewish year
pub fn year_length(year: i32) -> i32 {
    let rosh1 = year_start(year);
    let rosh2 = year_start(year+1);
    return rosh2-rosh1;
}

/// Determine if a Gregorian year is a leap year
pub fn greg_leap_year(year: i32) -> bool {
    return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Determine if a Gregorian date is valid
pub fn greg_valid(d: Date) -> bool {
    if d.month < 1 || d.month > 12 || d.day < 1 || d.day > 31 {
        return false
    }
    if d.day <= 30 {return true}
    match d.month {
        1|3|5|7|8|10|12 => true,
        4|6|9|11 => if d.day <= 30 {true} else {false},
        2 => if greg_leap_year(d.year) {d.day <= 29} else {d.day <= 28},
        _ => unreachable!()
    }
}

/// Converts a date in the proleptic Gregorian calendar to a Rata Die day number
///
/// Day 1 is January 1, 1. Please note the Gregorian calendar started in 1582
/// so don't rely on my calculations of dates before that.
pub fn greg_to_rd(d: Date) -> i32 {
    assert!(greg_valid(d));
    let (year, month, day) = (d.year, d.month, d.day);
    let mut leap_days = (year-1).div_euclid(4) - (year-1).div_euclid(100)
                      + (year-1).div_euclid(400);
    if greg_leap_year(year) && month >= 3 {
        leap_days += 1
    }
    let days_until_month = [0, 0, 31, 59, 90, 120, 151,
                            181, 212, 243, 273, 304, 334];
    return (year-1) * 365 + days_until_month[month as usize] + day as i32
        + leap_days;
}

pub fn from_rd(rd: i32) -> Date {
    let ed = 1373428 + rd; // days since epoch
    let mut year = ed * 100 / 36525;
    while year_start(year) < ed {year += 1;}
    while year_start(year) > ed {year -= 1;}
    let mut days = year_start(year);
    let mut days_in_month = [0, 30, 29, 30, 29, 30, 29,
                             30, 29, 30, 29, 30, 29, 0];
    if is_leap_year(year) {
        days_in_month[12] = 30; days_in_month[13] = 29;
    }
    let length = year_length(year);
    if length % 10 == 3 {
        // Deficient year
        days_in_month[9] = 29;
    }
    if length % 10 == 5 {
        // Complete year
        days_in_month[8] = 30;
    }
    let mut month = 7;
    loop {
        let length = days_in_month[month];
        if days + length > ed {break}
        month += 1;
        if month == 14 {month = 1}
        if month == 7 {unreachable!()}
        days += length;
    }
    return Date{
        year: year,
        month: month as u8,
        day: (ed-days+1) as u8
    };
}

pub fn from_greg(d: Date) -> Date {
    return from_rd(greg_to_rd(d));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_molad_year() {
        let tod = today();
        println!("{:?}", tod);
        assert_eq!(molad_components(1), (1, 5, 204));
        assert_eq!(molad_components(5785), (2112590, 9, 391));
    }

    #[test]
    fn test_leap_year() {
        assert!(is_leap_year(5700));
        assert!(!is_leap_year(5701));
        assert!(!is_leap_year(5702));
        assert!(is_leap_year(5703));
        assert!(is_leap_year(5782));
        assert!(!is_leap_year(5783));
        assert!(is_leap_year(5784));
        assert!(!is_leap_year(5785));
        assert!(!is_leap_year(5786));
        assert!(is_leap_year(5787));
    }

    #[test]
    fn test_from_greg() {
        assert_eq!(from_greg(1, 1, 1), (3761, 10, 18));
        assert_eq!(from_greg(-3760, 9,  7), (1, 7, 1));
        assert_eq!(from_greg(2024, 12, 31), (5785, 9, 30));
        assert_eq!(from_greg(2025,  1,  1), (5785, 10, 1));
        assert_eq!(from_greg(2025,  2,  1), (5785, 11, 3));
        assert_eq!(from_greg(2025,  3,  1), (5785, 12, 1));
        assert_eq!(from_greg(2024,  2, 10), (5784, 12, 1));
        assert_eq!(from_greg(2024,  3, 11), (5784, 13, 1));
        assert_eq!(from_greg(2024,  4,  9), (5784, 1, 1));
        assert_eq!(from_greg(2024, 10,  2), (5784, 6, 29));
        assert_eq!(from_greg(2024, 10,  3), (5785, 7, 1));
    }
}
