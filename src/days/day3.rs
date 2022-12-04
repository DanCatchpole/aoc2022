use std::fs::read_to_string;

// Converts character to a priority value:
// a..z => 1..26, A..Z => 27..52
fn char_to_priority(c: char) -> i32 {
    let offset = if c.is_uppercase() { ('A' as i32) - 27 } else { ('a' as i32) - 1 };
    return (c as i32) - offset;
}

// For a given character c, check that it is present in all rucksacks
fn contained_in_all(c: char, rucksacks: &Vec<&str>) -> bool {
    rucksacks.iter().all(|s| s.contains(c))
}

// For each character in the first rucksack, check the other rucksacks to sees
// if it is contained in them all
fn common_priority(rucksacks: Vec<&str>) -> i32 {
    for char in rucksacks[0].chars() {
        if contained_in_all(char, &rucksacks) {
            return char_to_priority(char);
        }
    }
    return -1;
}

// Each rucksack is split in half to form 2 separate rucksacks
// Find the priority of the item that is present in both halves
fn part_a(filename: &str) -> i32 {
    let contents = read_to_string(filename).expect("Could not read file");
    contents
        .lines()
        .map(|sack| {
            let halves = sack.split_at(sack.len() / 2);
            return common_priority(vec![halves.0, halves.1]);
        })
        .sum()
}

// Check each set of 3 rucksacks to find the priority of the shared item
fn part_b(filename: &str) -> i32 {
    let contents = read_to_string(filename).expect("Could not read file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut count = 0;
    for i in (0..lines.len()).step_by(3) {
        count += common_priority(vec![lines[i], lines[i + 1], lines[i + 2]]);
    }
    return count;
}

pub fn exec() {
    let filename = "input/day3.txt";
    crate::utils::output_results(part_a(filename), part_b(filename));
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_FILE_NAME: &str = "test/day3.txt";

    #[test]
    fn should_convert_upper_character_to_priority() {
        assert_eq!(char_to_priority('A'), 27);
    }

    #[test]
    fn should_convert_lower_character_to_priority() {
        assert_eq!(char_to_priority('c'), 3);
    }

    #[test]
    fn should_do_part_a() {
        assert_eq!(part_a(TEST_FILE_NAME), 157);
    }

    #[test]
    fn should_do_part_b() {
        assert_eq!(part_b(TEST_FILE_NAME), 70);
    }
}