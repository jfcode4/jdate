use jdate::*;

fn main() {
    println!("{}", from_greg(Date::from(1, 1, 1)));
    molad_print(5785);
    molad_print(1);
    println!("2024-02-28 = {}", from_greg(Date::from(2024, 2, 28)));
    println!("2024-02-29 = {}", from_greg(Date::from(2024, 2, 29)));
    println!("-3760-09-07 = {}", from_greg(Date::from(-3760, 9, 7)));
    for i in 1900..2030 {
        let date = Date::from(i, 1, 1);
        println!("{} = {}", date, from_greg(date));
    }
    let today = today();
    println!("Today is: {} = {}", today, from_greg(today));
}
