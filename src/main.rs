use jdate::*;
use time::Date;

fn main() {
    molad_print(5785);
    molad_print(1);

    let date = gdate(1, 1, 1).unwrap();
    println!("{}", JDate::from(date));
    let date = gdate(2024, 2, 28).unwrap();
    println!("2024-02-28 = {}", JDate::from(date));
    let date = gdate(2024, 2, 29).unwrap();
    println!("2024-02-29 = {}", JDate::from(date));
    let date = gdate(-3760, 9, 7).unwrap();
    println!("-3760-09-07 = {}", JDate::from(date));
    for i in 1900..2030 {
        let date = gdate(i, 1, 1).unwrap();
        println!("{} = {}", date, JDate::from(date));
    }

    let today = today();
    println!("Today is: {} = {}", today, JDate::from(today));
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 4 {
        let year: i32 = args[1].parse().expect("Invalid year");
        let month: u8 = args[2].parse().expect("Invalid month");
        let day: u8 = args[3].parse().expect("Invalid day");
        let date = JDate::new(year, month, day).expect("Invalid date");
        println!("The given date is: {}", date);
        println!("Which is {} in Gregorian date.", Date::from(date));
    }

}
