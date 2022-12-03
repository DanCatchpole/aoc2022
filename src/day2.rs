#![allow(dead_code)]

use std::fs::read_to_string;
use num_enum::{ TryFromPrimitive, IntoPrimitive };

#[derive(PartialEq, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
enum Choice {
    ROCK = 0,
    PAPER = 1,
    SCISSORS = 2,
}

enum Result {
    LOSE,
    DRAW,
    WIN,
}

fn score(pair: &(Choice, Choice)) -> i32 {
    let match_point = match pair {
        (a, b) if a == b => 3,
        | (Choice::ROCK, Choice::PAPER)
        | (Choice::PAPER, Choice::SCISSORS)
        | (Choice::SCISSORS, Choice::ROCK) => 6,
        (_, _) => 0,
    };
    let choice_point: i32 = pair.1.into();
    return match_point + choice_point + 1;
}

fn loses(choice: Choice) -> Choice {
    let num: i32 = choice.into();
    return Choice::try_from((num + 2) % 3).expect("Could not convert");
}

fn beats(choice: Choice) -> Choice {
    let num: i32 = choice.into();
    return Choice::try_from((num + 1) % 3).expect("Could not convert");
}

fn get_play(tuple: &(Choice, Result)) -> (Choice, Choice) {
    match *tuple {
        (a, Result::DRAW) => (a, a),
        (a, Result::WIN) => (a, beats(a)),
        (a, Result::LOSE) => (a, loses(a)),
    }
}

fn get_choice(string: &str) -> Choice {
    match string {
        "A" | "X" => Choice::ROCK,
        "B" | "Y" => Choice::PAPER,
        "C" | "Z" => Choice::SCISSORS,
        &_ => panic!("Invalid value"),
    }
}

fn get_result(string: &str) -> Result {
    match string {
        "X" => Result::LOSE,
        "Y" => Result::DRAW,
        "Z" => Result::WIN,
        &_ => panic!("Invalid value"),
    }
}

fn parse_string(string: &str) -> (Choice, Choice) {
    let splt: Vec<&str> = string.split(" ").collect();
    return (get_choice(splt[0]), get_choice(splt[1]));
}

fn parse_result(string: &str) -> (Choice, Result) {
    let splt: Vec<&str> = string.split(" ").collect();
    return (get_choice(splt[0]), get_result(splt[1]));
}

fn part_a(filename: &str) -> i32 {
    let contents = read_to_string(filename).expect("Could not read file");
    let input: Vec<(Choice, Choice)> = contents.lines().map(parse_string).collect();
    input.iter().map(score).sum()
}

fn part_b(filename: &str) -> i32 {
    let contents = read_to_string(filename).expect("Could not read file");
    let input: Vec<(Choice, Result)> = contents.lines().map(parse_result).collect();
    let plays: Vec<(Choice, Choice)> = input.iter().map(get_play).collect();
    plays.iter().map(score).sum()
}

pub fn test() {
    let filename = "./test/day2.txt";
    let result_a = part_a(filename);
    let result_b = part_b(filename);
    assert!(result_a == 15, "Expected 15, Got {}", result_a);
    assert!(result_b == 12, "Expected 12, Got {}", result_b);
    println!("Test passed");
}

pub fn exec() {
    let filename = "./input/day2.txt";
    println!("Part A: {}", part_a(filename));
    println!("Part B: {}", part_b(filename));
}