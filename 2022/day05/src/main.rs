use std::{collections::VecDeque, error::Error, fs};

fn parse_crates(s: Vec<String>, n: usize, height: usize) -> Vec<VecDeque<char>> {
    let mut vec: Vec<VecDeque<char>> = vec![VecDeque::new(); n];
    for i in (0..height).rev() {
        for (j, stack) in vec.iter_mut().enumerate().take(n) {
            let c = s[i].chars().nth(1 + j * 4).unwrap();
            if c != ' ' {
                stack.push_back(c);
            }
        }
    }
    vec
}

fn parse_moves(data: Vec<String>) -> Vec<(usize, usize, usize)> {
    data.iter()
        .map(|s| {
            let x: Vec<&str> = s.split(' ').collect();
            let nums: Vec<usize> = vec![x[1], x[3], x[5]]
                .iter()
                .map(|num| num.parse::<usize>().unwrap())
                .collect();
            (nums[0], nums[1] - 1, nums[2] - 1)
        })
        .collect()
}

fn task_1(crates: &Vec<VecDeque<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    let mut clone = crates.clone();
    for (amount, a, b) in moves {
        // Move 'amount' from 'a' to 'b'
        for _ in 0..*amount {
            match clone[*a].pop_back() {
                Some(x) => clone[*b].push_back(x),
                None => break,
            }
        }
    }
    String::from_iter(clone.iter().map(|v| *v.back().unwrap()))
}

fn task_2(crates: &Vec<VecDeque<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    let mut clone = crates.clone();
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
    String::from_iter(clone.iter().map(|v| *v.back().unwrap()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let text: Vec<String> = fs::read_to_string("input.txt")?
        .split('\n')
        .map(String::from)
        .collect();
    let index = text.iter().position(|s| s.starts_with(" 1")).unwrap();
    let n_str = text[index].split("   ").last().unwrap();
    let n = n_str.trim().parse().unwrap();
    let crates = parse_crates(text[0..index].to_vec(), n, index);
    let moves = parse_moves(text[index + 2..text.len()].to_vec());
    println!("task 1: {}", task_1(&crates, &moves));
    println!("task 2: {}", task_2(&crates, &moves));
    Ok(())
}
