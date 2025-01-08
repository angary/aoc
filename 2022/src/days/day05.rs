use regex::Regex;
use std::{collections::VecDeque, error::Error};

type Move = (usize, usize, usize);

fn parse_crates(crate_lines: &[String], n: usize) -> Vec<VecDeque<char>> {
    let mut crates = vec![VecDeque::new(); n];
    for line in crate_lines.iter().rev() {
        for (i, stack) in crates.iter_mut().enumerate() {
            let index = 1 + i * 4;
            let c = line.chars().nth(index).unwrap();
            if c != ' ' {
                stack.push_back(c);
            }
        }
    }
    crates
}

fn parse_moves(move_instructions: &[String]) -> Result<Vec<Move>, Box<dyn Error>> {
    let move_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
    let moves = move_instructions
        .iter()
        .filter_map(|line| {
            move_regex.captures(line).map(|caps| {
                let amount = caps[1].parse::<usize>().unwrap();
                let from = caps[2].parse::<usize>().unwrap() - 1; // convert to 0-indexed
                let to = caps[3].parse::<usize>().unwrap() - 1; // convert to 0-indexed
                (amount, from, to)
            })
        })
        .collect();
    Ok(moves)
}

fn task_1(crates: &[VecDeque<char>], moves: &[Move]) -> String {
    let mut clone = crates.to_owned();
    for (amount, a, b) in moves {
        // Move 'amount' from 'a' to 'b'
        for _ in 0..*amount {
            match clone[*a].pop_back() {
                Some(x) => clone[*b].push_back(x),
                None => break,
            }
        }
    }
    clone.iter().filter_map(|v| v.back()).collect()
}

fn task_2(crates: &[VecDeque<char>], moves: &[Move]) -> String {
    let mut clone = crates.to_owned();
    for (amount, a, b) in moves {
        // Move 'amount' from 'a' to 'b'
        let mut moved: VecDeque<char> = VecDeque::new();
        for _ in 0..*amount {
            match clone[*a].pop_back() {
                Some(x) => moved.push_front(x),
                None => break,
            }
        }
        clone[*b].append(&mut moved)
    }
    clone.iter().filter_map(|v| v.back()).collect()
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let text: Vec<String> = input.lines().map(String::from).collect();
    let index = text.iter().position(|s| s.starts_with(" 1")).unwrap();
    let crate_lines = &text[0..index];
    let move_lines = &text[index + 2..];

    let n_str = text[index].split_whitespace().last().unwrap();
    let n = n_str.trim().parse().unwrap();
    let crates = parse_crates(crate_lines, n);
    let moves = parse_moves(move_lines)?;

    println!("task 1: {}", task_1(&crates, &moves));
    println!("task 2: {}", task_2(&crates, &moves));
    Ok(())
}
