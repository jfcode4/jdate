use jdate::*;

fn main() {
    // Convert a gregorian date to Jewish date
    let date1 = Date::from(2025, 1, 1);
    let date2 = from_greg(date1);
    println!("{date2}"); // prints: 5785-10-01

    // Convert today's date
    let todays_date = from_greg(today());
    println!("{todays_date}");
}
