use std::collections::HashSet;
use std::fs::read_to_string;
use std::string::String;

use crate::utils::output_results;

fn all_unique(string: &str) -> bool {
    let chars: HashSet<char> = HashSet::from_iter(string.chars());
    chars.len() == string.len()
}

fn find_position(content: String, num_distinct: i32) -> i32 {
    for i in (num_distinct as usize) - 1..content.len() {
        let substr = &content[i - ((num_distinct as usize) - 1)..i + 1];
        if all_unique(substr) {
            return (i as i32) + 1;
        }
    }
    return -1;
}

fn part_a(content: String) -> i32 {
    find_position(content, 4)
}

fn part_b(content: String) -> i32 {
    find_position(content, 14)
}

pub fn exec() {
    let content = read_to_string("input/day6.txt").expect("Could not read file");
    output_results(part_a(content.clone()), part_b(content));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_do_part_a() {
        assert_eq!(part_a(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(part_a(String::from("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(part_a(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
        assert_eq!(part_a(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
    }

    #[test]
    fn should_do_part_b() {
        assert_eq!(part_b(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
        assert_eq!(part_b(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(part_b(String::from("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(part_b(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 29);
        assert_eq!(part_b(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
    }
}