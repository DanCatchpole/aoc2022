#![allow(dead_code)]

pub fn replace_all(mut source: String, occurences: Vec<&str>, replace_with: &str) -> String {
    for occur in occurences {
        source = source.replace(occur, replace_with);
    }
    return source;
}

pub fn str_to_int(string: &str) -> i32 {
    string.parse::<i32>().expect("Couldn't parse")
}

pub fn output_results(a: i32, b: i32) {
    println!(" - Part A: {}", a);
    println!(" - Part B: {}", b);
}