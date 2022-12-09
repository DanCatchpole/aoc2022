use std::{ fs::read_to_string, collections::HashSet };

use crate::utils::output_results;

fn cd(dir: &mut Vec<String>, command: &str) {
    if command == "$ cd .." {
        dir.pop();
    } else {
        dir.push(String::from(command.split_at(5).1) + "/")
    }
}

fn full_path(dir: &Vec<String>, filename: &str) -> String {
    return dir.concat() + filename;
}

fn parse(filename: &str) -> (Vec<(String, i64)>, HashSet<String>) {
    let content = read_to_string(filename).expect("Could not read file");

    let mut files: Vec<(String, i64)> = vec![];
    let mut dirs: HashSet<String> = HashSet::from([String::from("/")]);

    let mut current_directory: Vec<String> = vec![String::from("/")];
    for line in content.lines().skip(1) {
        if line.starts_with("$ cd") {
            cd(&mut current_directory, line);
            dirs.insert(String::from(&current_directory.concat()));
        } else if !line.starts_with("dir") && !line.starts_with("$") {
            // the line is a file
            let split: Vec<&str> = line.split(" ").collect();
            files.push((
                full_path(&current_directory, split[1]),
                split[0].parse::<i64>().expect("Failed to parse int"),
            ));
        }
    }
    return (files, dirs);
}

fn sum_files(prefix: &String, files: &Vec<(String, i64)>) -> i64 {
    files
        .iter()
        .filter_map(|file| {
            if file.0.starts_with(prefix) {
                return Some(file.1)
            } else  {
                return None
            }
        })
        .sum()
}

fn get_sizes(files: Vec<(String, i64)>, dirs: HashSet<String>) -> Vec<i64> {
    dirs.iter()
        .map(|dir| { sum_files(dir, &files) })
        .collect()
}

fn part_a(sizes: &Vec<i64>) -> i64 {
    sizes
        .iter()
        .filter(|e| **e <= 100000)
        .sum()
}

fn part_b(mut sizes: Vec<i64>) -> i64 {
    sizes.sort();
    let unused_space = 70000000 - sizes[sizes.len() - 1];
    *sizes
        .iter()
        .filter(|s| { **s > 30000000 - unused_space })
        .next().expect("No candidates found")
}

pub fn exec() {
    let (files, dirs) = parse("input/day7.txt");
    let sizes = get_sizes(files, dirs);
    output_results(part_a(&sizes) as i32, part_b(sizes) as i32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_cd_up() {
        let mut dir = vec![String::from("/"), String::from("abc/")];
        cd(&mut dir, "$ cd ..");
        assert_eq!(dir, vec![String::from("/")])
    }

    #[test]
    fn do_cd_down() {
        let mut dir = vec![String::from("/"), String::from("abc/")];
        cd(&mut dir, "$ cd def");
        assert_eq!(dir, vec![String::from("/"), String::from("abc/"), String::from("def/")])
    }

    #[test]
    fn do_parse() {
        let (parsed, dirs) = parse("test/day7_short.txt");
        assert_eq!(
            parsed,
            vec![(String::from("/b.txt"), 14848514), (String::from("/c.dat"), 8504156)]
        );
        assert_eq!(dirs, HashSet::from([String::from("/")]))
    }

    #[test]
    fn do_parse_long() {
        let (_, dirs) = parse("test/day7.txt");
        assert_eq!(
            dirs,
            HashSet::from([
                String::from("/"),
                String::from("/a/"),
                String::from("/a/e/"),
                String::from("/d/"),
            ])
        )
    }

    #[test]
    fn do_part_a() {
        let (files, dirs) = parse("test/day7.txt");
        let sizes = get_sizes(files, dirs);
        assert_eq!(part_a(&sizes), 95437)
    }

    #[test]
    fn do_part_b() {
        let (files, dirs) = parse("test/day7.txt");
        let sizes = get_sizes(files, dirs);
        assert_eq!(part_b(sizes), 24933642)
    }
}