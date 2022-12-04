use std::env;

mod utils;
mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "all" {
        days::run_all();
    } else {
        let day = args[1].parse::<i32>().expect("Need to enter a day number");
        days::run_day(day);    
    }
}