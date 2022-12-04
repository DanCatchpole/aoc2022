use std::fs::read_to_string;

fn parse_input(filename: &str) -> Vec<i32> {
    let file_content = read_to_string(filename).expect("Could not read file");
    let lines: Vec<&str> = file_content.lines().collect();

    let mut elves = vec![0];
    for line in lines {
        if line == "" {
            elves.push(0);
        } else {
            let end = elves.len() - 1;
            elves[end] += line.parse::<i32>().unwrap();
        }
    }
    return elves;
}

fn part_a(elves: Vec<i32>) -> i32 {
    match elves.iter().max() {
        Some(max) => *max,
        None => -1,
    }
}

fn part_b(mut elves: Vec<i32>) -> i32 {
    elves.sort_by(|a, b| b.cmp(a));
    return elves[0] + elves[1] + elves[2];
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_FILE_NAME: &str = "test/day1.txt";

    #[test]
    fn should_do_part_a() {
        let elves = parse_input(TEST_FILE_NAME);
        assert_eq!(part_a(elves), 24000);
    }

    #[test]
    fn should_do_part_b() {
        let elves = parse_input(TEST_FILE_NAME);
        assert_eq!(part_b(elves), 45000);
    }
}

pub fn exec() {
    let elves = parse_input("./input/day1.txt");
    crate::utils::output_results(part_a(elves.clone()), part_b(elves.clone()));
}