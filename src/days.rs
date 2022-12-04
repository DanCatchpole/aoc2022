pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub fn run_day(day: i32) {
    match day {
        1 => day1::exec(),
        2 => day2::exec(),
        3 => day3::exec(),
        4 => day4::exec(),
        _ => println!("No such day")
    }
}