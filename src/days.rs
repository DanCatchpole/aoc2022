use std::time::Instant;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub fn run_day(day: i32) {
    println!("Running day {}", day);
    let now = Instant::now();
    match day {
        1 => day1::exec(),
        2 => day2::exec(),
        3 => day3::exec(),
        4 => day4::exec(),
        _ => println!("No such day")
    }
    println!("Executed in {}ms", (now.elapsed().as_millis()));
}

pub fn run_all() {
    for i in 1..5 {
        run_day(i);
        println!("");
    }
}