use std::time::Instant;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub fn run_day(day: i32) {
    println!("Day {}", day);
    let now = Instant::now();
    match day {
        1 => day1::exec(),
        2 => day2::exec(),
        3 => day3::exec(),
        4 => day4::exec(),
        5 => day5::exec(),
        6 => day6::exec(),
        7 => day7::exec(),
        _ => println!("No such day")
    }
    println!(" - Executed in {}μs", (now.elapsed().as_micros()));
}

pub fn run_all() {
    for i in 1..8 {
        run_day(i);
        println!("");
    }
}