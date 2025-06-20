# jdate

This is my attempt at writing a Jewish calendar converter in Rust. This was
inspired by the hebcal project.

## Features

- Convert a gregorian date to a Jewish date
- Convert a jewish date to a gregorian date
- Calculate the molad of the start of a year

## Example
```rust
use jdate::JDate;
use time::Date;

fn main() {
    // Convert a Gregorian date to a Jewish date
    let date1 = jdate::gdate(2025, 1, 1).unwrap();
    let date2 = JDate::from(date1);
    println!("{date2}"); // prints: 5785-Tevet-01
                         //
    // Convert a Jewish date to a Gregorian date
    let date1 = JDate::new(5785, 1, 1).unwrap();
    let date2 = Date::from(date1);
    println!("{date2}"); // prints: 2025-03-30

    // Convert today's date
    let today = jdate::today();
    let today_jdate = JDate::from(today);
    println!("Today is {today} = {today_jdate}");
}
```
