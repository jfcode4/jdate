use time::{OffsetDateTime, Date};
use std::fmt;

// The Epoch we use is the molad tohu (Day 1 = 1 Tishrei 1 = 7 September -3760)
// It is a theoretical time point 1 year before creation.


/// An easier way to create a time::Date object
pub fn gdate(year: i32, month: u8, day: u8) -> Option<Date> {
    return Date::from_calendar_date(year, month.try_into().unwrap(), day).ok();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JDate {
    year: i32,
    month: u8, // 1 = Nisan, 13 = Adar2
    day: u8, // 1 - 30
}

impl JDate {
    pub fn new(year: i32, month: u8, day: u8) -> Option<JDate> {
        if !date_is_valid(year, month, day) {
            return None;
        }
        Some(JDate{year, month, day})
    }

    pub fn from_jd(jd: i32) -> JDate {
        let ed = jd - 347997; // days since epoch
        let mut year = ed * 100 / 36525;
        while year_start(year) < ed {year += 1;}
        while year_start(year) > ed {year -= 1;}
        let mut days = year_start(year);
        let days_in_month = year_months(year);
        let mut month = 7;
        loop {
            let length = days_in_month[month] as i32;
            if days + length > ed {break}
            month += 1;
            if month == 14 {month = 1}
            if month == 7 {unreachable!()}
            days += length;
        }
        return JDate{
            year: year,
            month: month as u8,
            day: (ed-days+1) as u8
        };
    }
    pub fn to_jd(self: Self) -> i32 {
        let mut ed = year_start(self.year) - 1;
        let days_in_month = year_months(self.year);
        let mut month: u8 = 7;
        loop {
            let length = days_in_month[month as usize];
            if month == self.month {
                ed += self.day as i32;
                break;
            }
            ed += length as i32;
            month += 1;
            if month == 14 {month = 1}
            if month == 7 {unreachable!()}
        }
        return ed + 347997;
    }


    pub fn year(self: Self) -> i32 {return self.year}
    pub fn month(self: Self) -> u8 {return self.month}
    pub fn day(self: Self) -> u8 {return self.day}

    pub fn month_name(self: Self) -> &'static str {
        const NAMES: [&str; 13] = [
            "Nisan", "Iyar", "Sivan", "Tamuz", "Av", "Elul",
            "Tishrei", "Cheshvan", "Kislev", "Tevet", "Shvat", "Adar", "Adar2"];
        if self.month == 12 && is_leap_year(self.year) {
            return "Adar1"
        }
        return NAMES[self.month as usize - 1];

    }
}

impl fmt::Display for JDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>4}-{}-{:0>2}", self.year, self.month_name(), self.day)
    }
}

impl From<Date> for JDate {
    /// Convert Gregorian Date to JDate
    fn from(d: Date) -> Self {
        let jd = d.to_julian_day();
        return JDate::from_jd(jd);
    }
}

impl From<JDate> for Date {
    /// Convert JDate to Gregorian Date
    fn from(d: JDate) -> Self {
        let jd = d.to_jd();
        return Date::from_julian_day(jd).unwrap();
    }
}

/// Get today's local date
pub fn today() -> Date {
    let now = OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc());
    return now.date();
}

/// Determine if the Jewish year is a leap year
pub fn is_leap_year(year: i32) -> bool {
    match year % 19 {
        0|3|6|8|11|14|17 => true,
        _ => false
    }
}

/// Determine if the Jewish date is valid
pub fn date_is_valid(year: i32, month: u8, day: u8) -> bool {
    if month < 1 || month > 13 || day < 1 || day > 30 {
        return false;
    }
    if month == 13 {
        return day <= 29 && is_leap_year(year);
    }
    if day == 30 {
        return match month {
            1|3|5|7|11 => true,
            2|4|6|10 => false,
            12 => is_leap_year(year),
            8 => {
                let len = year_length(year);
                len % 10 == 5 // complete year (355 or 385)
            },
            9 => {
                let len = year_length(year);
                len % 10 >= 4 // complete or regular year (354 or 384)
            },
            _ => unreachable!()
        }
    }
    true
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

// Calculates the number of days in the Jewish year
pub fn year_length(year: i32) -> i32 {
    let rosh1 = year_start(year);
    let rosh2 = year_start(year+1);
    return rosh2-rosh1;
}

/// Returns a list of months with the number of days in them
pub fn year_months(year: i32) -> [u8; 14] {
    let mut days_in_month = [0, 30, 29, 30, 29, 30, 29,     // Nisan - Elul
                                30, 29, 30, 29, 30, 29, 0]; // Tishrei - Adar2
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
    return days_in_month;
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
        assert_eq!(JDate::from(gdate(1, 1, 1).unwrap()),
                   JDate::new(3761, 10, 18).unwrap());
        assert_eq!(JDate::from(gdate(-3760, 9,  7).unwrap()),
                   JDate::new(1, 7, 1).unwrap());
        assert_eq!(JDate::from(gdate(2024, 12, 31).unwrap()),
                   JDate::new(5785, 9, 30).unwrap());
        assert_eq!(JDate::from(gdate(2025,  1,  1).unwrap()),
                   JDate::new(5785, 10, 1).unwrap());
        assert_eq!(JDate::from(gdate(2025,  2,  1).unwrap()),
                   JDate::new(5785, 11, 3).unwrap());
        assert_eq!(JDate::from(gdate(2025,  3,  1).unwrap()),
                   JDate::new(5785, 12, 1).unwrap());
        assert_eq!(JDate::from(gdate(2024,  2, 10).unwrap()),
                   JDate::new(5784, 12, 1).unwrap());
        assert_eq!(JDate::from(gdate(2024,  3, 11).unwrap()),
                   JDate::new(5784, 13, 1).unwrap());
        assert_eq!(JDate::from(gdate(2024,  4,  9).unwrap()),
                   JDate::new(5784, 1, 1).unwrap());
        assert_eq!(JDate::from(gdate(2024, 10,  2).unwrap()),
                   JDate::new(5784, 6, 29).unwrap());
        assert_eq!(JDate::from(gdate(2024, 10,  3).unwrap()),
                   JDate::new(5785, 7, 1).unwrap());
    }

    #[test]
    fn test_from_jdate() {
        assert_eq!(Date::from(JDate::new(3761, 10, 18).unwrap()),
                   gdate(1, 1, 1).unwrap());
        assert_eq!(Date::from(JDate::new(1, 7, 1).unwrap()),
                   gdate(-3760, 9, 7).unwrap());
        assert_eq!(Date::from(JDate::new(5785, 9, 30).unwrap()),
                   gdate(2024, 12, 31).unwrap());
        assert_eq!(Date::from(JDate::new(5785, 10, 1).unwrap()),
                   gdate(2025, 1, 1).unwrap());
        assert_eq!(Date::from(JDate::new(5785, 11, 3).unwrap()),
                   gdate(2025, 2, 1).unwrap());
        assert_eq!(Date::from(JDate::new(5785, 12, 1).unwrap()),
                   gdate(2025, 3, 1).unwrap());
        assert_eq!(Date::from(JDate::new(5784, 12, 1).unwrap()),
                   gdate(2024, 2, 10).unwrap());
        assert_eq!(Date::from(JDate::new(5784, 13, 1).unwrap()),
                   gdate(2024, 3, 11).unwrap());
        assert_eq!(Date::from(JDate::new(5784, 1, 1).unwrap()),
                   gdate(2024, 4, 9).unwrap());
        assert_eq!(Date::from(JDate::new(5784, 6, 29).unwrap()),
                   gdate(2024, 10, 2).unwrap());
        assert_eq!(Date::from(JDate::new(5785, 7, 1).unwrap()),
                   gdate(2024, 10, 3).unwrap());
    }
}
