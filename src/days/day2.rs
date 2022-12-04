use std::fs::read_to_string;
use crate::utils;

// Rock = 1, Paper = 2, Scissors = 3
// Loss = 0, Draw = 1, Loss = 2
// (me - you) % 3 = result
// (1 + me - you) % 3 => 0 for loss, 1 for draw, 2 for win
// * 3 => 0 loss, 3 draw, 6 win
fn part_a(filename: &str) -> i32 {
    let mut contents = read_to_string(filename).expect("Could not read file");
    contents = utils::replace_all(contents, vec!["A", "X"], "1");
    contents = utils::replace_all(contents, vec!["B", "Y"], "2");
    contents = utils::replace_all(contents, vec!["C", "Z"], "3");

    let input: Vec<i32> = contents
        .lines()
        .map(|x: &str| {
            let parts: Vec<i32> = x.split(" ").map(utils::str_to_int).collect();
            parts[1] + ((4 + parts[1] - parts[0]) % 3) * 3 // add 4 instead of 1 as % is technically remainder
        })
        .collect();
    input.iter().sum()
}

// Rock = 1, Paper = 2, Scissors = 3
// Loss = 1, Draw = 2, Win = 3 (result)
// (1 + me - you) = (result - 1) (% 3)
// me = (1 + result + you) % 3
// score = (result - 1) * 3
fn part_b(filename: &str) -> i32 {
    let mut contents = read_to_string(filename).expect("Could not read file");
    contents = utils::replace_all(contents, vec!["A", "X"], "1");
    contents = utils::replace_all(contents, vec!["B", "Y"], "2");
    contents = utils::replace_all(contents, vec!["C", "Z"], "3");

    let input: Vec<i32> = contents
        .lines()
        .map(|x: &str| {
            let parts: Vec<i32> = x.split(" ").map(utils::str_to_int).collect();
            let my_score = 1 + ((parts[1] + parts[0]) % 3);
            let score = (parts[1] - 1) * 3;
            my_score + score
        })
        .collect();
    input.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_FILE_NAME: &str = "test/day2.txt";

    #[test]
    fn should_do_part_a() {
        assert_eq!(part_a(TEST_FILE_NAME), 15);
    }

    #[test]
    fn should_do_part_b() {
        assert_eq!(part_b(TEST_FILE_NAME), 12);
    }
}

pub fn exec() {
    let filename = "./input/day2.txt";
    println!("Part A: {}", part_a(filename));
    println!("Part B: {}", part_b(filename));
}