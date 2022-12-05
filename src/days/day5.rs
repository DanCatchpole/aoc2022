use std::fs::read_to_string;

use crate::utils::output_results_str;

fn parse_string(string: &str, cols: &mut Vec<Vec<char>>) {
    for (i, v) in string.chars().skip(1).step_by(4).enumerate() {
        if v != ' ' {
            cols[i].push(v);
        }
    }
}

fn parse_movement_string(string: &str) -> (i32, usize, usize) {
    let parts: Vec<&str> = string.split(" ").collect();
    (
        parts[1].parse::<i32>().expect("Could not parse int"),
        parts[3].parse::<usize>().expect("could not parse"),
        parts[5].parse::<usize>().expect("could not parse"),
    )
}

fn parse(filename: &str, num_cols: i32) -> (Vec<Vec<char>>, Vec<(i32, usize, usize)>) {
    let contents = read_to_string(filename).expect("Could not read file");

    let mut cols: Vec<Vec<char>> = vec![];
    let mut moves: Vec<(i32, usize, usize)> = vec![];
    for _ in 0..num_cols {
        cols.push(vec![]);
    }
    for line in contents.lines() {
        if line == "" || line.starts_with(" 1") {
            // either empty line or the column labels, discard
            continue;
        }
        if line.starts_with("move") {
            // parse movement instructions
            moves.push(parse_movement_string(line));
        } else {
            // parse grid
            parse_string(line, &mut cols);
        }
    }

    for c in 0..num_cols {
        cols[c as usize].reverse();
    }
    (cols, moves)
}

fn get_tops(cols: &mut Vec<Vec<char>>) -> String {
    cols.iter()
        .map(|col| col[col.len() - 1])
        .collect::<String>()
}

fn part_a(cols: &mut Vec<Vec<char>>, moves: Vec<(i32, usize, usize)>) -> String {
    for (amount, c1, c2) in moves.iter() {
        for _ in 0..*amount {
            let char = cols[*c1 - 1].pop().expect("vec was empty");
            cols[*c2 - 1].push(char);
        }
    }
    get_tops(cols)
}

fn part_b(cols: &mut Vec<Vec<char>>, moves: Vec<(i32, usize, usize)>) -> String {
    for (amount, c1, c2) in moves.iter() {
        let mut temp_vec: Vec<char> = vec![];
        for _ in 0..*amount {
            let char = cols[*c1 - 1].pop().expect("vec was empty");
            temp_vec.push(char);
        }
        temp_vec.reverse();
        for i in 0..temp_vec.len() {
            cols[*c2 - 1].push(temp_vec[i]);
        }
    }
    get_tops(cols)
}
pub fn exec() {
    let filename = "input/day5.txt";
    let (cols, moves) = parse(filename, 9);
    let part_a_ans = part_a(&mut cols.clone(), moves.clone());
    let part_b_ans = part_b(&mut cols.clone(), moves.clone());
    output_results_str(part_a_ans, part_b_ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_string() {
        let str = "[Z] [M] [P]";
        let mut cols = vec![vec![], vec![], vec![]];
        parse_string(str, &mut cols);
        assert_eq!(cols, vec![vec!['Z'], vec!['M'], vec!['P']]);
    }

    #[test]
    fn should_parse_broken_string() {
        let str = "[N] [C]    ";
        let mut cols = vec![vec![], vec![], vec![]];
        parse_string(str, &mut cols);
        assert_eq!(cols, vec![vec!['N'], vec!['C'], vec![]]);
    }

    #[test]
    fn should_parse_string_with_whitespace() {
        let str = "    [D]    ";
        let mut cols = vec![vec![], vec![], vec![]];
        parse_string(str, &mut cols);
        assert_eq!(cols, vec![vec![], vec!['D'], vec![]]);
    }

    #[test]
    fn should_parse_grid() {
        let filename = "test/day5.txt";
        let (cols, _) = parse(filename, 3);
        assert_eq!(cols, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
    }

    #[test]
    fn should_parse_moves() {
        let filename = "test/day5.txt";
        let (_, moves) = parse(filename, 3);
        assert_eq!(moves, vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)])
    }

    #[test]
    fn should_do_part_a() {
        let filename = "test/day5.txt";
        let (mut cols, moves) = parse(filename, 3);
        assert_eq!(part_a(&mut cols, moves), "CMZ");
    }

    #[test]
    fn should_do_part_b() {
        let filename = "test/day5.txt";
        let (mut cols, moves) = parse(filename, 3);
        assert_eq!(part_b(&mut cols, moves), "MCD");
    }
}