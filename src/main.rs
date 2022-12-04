use std::env;

mod utils;
mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<i32>().expect("Need to enter a day number");
    println!("Running day {}", day);
    days::run_day(day);
}