use std::fs::read_to_string;

fn contains(a: (i32, i32), b: (i32, i32)) -> bool {
    b.0 >= a.0 && b.0 <= a.1 && b.1 >= a.0 && b.1 <= a.1
}

fn overlaps(a: (i32, i32), b: (i32, i32)) -> bool {
    inside(a.0, b) || inside(a.1, b) || inside(b.0, a) || inside(b.1, a)
}

fn inside(a: i32, b: (i32, i32)) -> bool {
    a >= b.0 && a <= b.1
}

fn parse_input(filename: &str) -> Vec<((i32, i32), (i32, i32))> {
    let mut contents = read_to_string(filename).expect("Could not read file");
    contents = crate::utils::replace_all(contents, vec!["-"], ",");
    let allocations: Vec<((i32, i32), (i32, i32))> = contents
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split(",")
                .map(|n| n.parse::<i32>().expect("Could not parse int"))
                .collect();
            ((nums[0], nums[1]), (nums[2], nums[3]))
        })
        .collect();
    return allocations;
}

fn part_a(filename: &str) -> i32 {
    let allocations = parse_input(filename);
    let filtered: Vec<((i32, i32), (i32, i32))> = allocations
        .into_iter()
        .filter(|&p| (contains(p.0, p.1) || contains(p.1, p.0)))
        .collect();
    return filtered.len() as i32;
}

fn part_b(filename: &str) -> i32 {
    let allocations = parse_input(filename);
    let filtered: Vec<((i32, i32), (i32, i32))> = allocations
        .into_iter()
        .filter(|&p| overlaps(p.0, p.1))
        .collect();

    return filtered.len() as i32;
}

pub fn exec() {
    let filename = "input/day4.txt";
    crate::utils::output_results(part_a(filename), part_b(filename));
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_FILE_NAME: &str = "test/day4.txt";

    #[test]
    fn should_do_part_a() {
        assert_eq!(part_a(TEST_FILE_NAME), 2);
    }

    #[test]
    fn should_do_part_b() {
        assert_eq!(part_b(TEST_FILE_NAME), 4);
    }

    #[test]
    fn larger_contains_smaller() {
        assert_eq!(contains((4, 6), (6, 6)), true);
    }

    #[test]
    fn smaller_does_not_contain_larger() {
        assert_eq!(contains((3, 7), (2, 8)), false);
    }

    #[test]
    fn does_overlap() {
        assert_eq!(overlaps((1, 3), (3, 5)), true);
    }

    #[test]
    fn does_not_overlap() {
        assert_eq!(overlaps((1, 3), (5, 9)), false);
    }
}