use std::error::Error;

use itertools::Itertools;
use regex::Regex;

struct Monkey {
    items: Vec<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(i32) -> usize>,
}

fn parse_starting_items(line: &str) -> Vec<i32> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(line)
        .flat_map(|m| m.as_str().parse())
        .collect()
}

fn get_last_number(line: &str) -> i32 {
    line.split_whitespace().last().unwrap().parse().unwrap()
}

fn parse_operation(line: &str) -> Box<dyn Fn(i32) -> i32> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let is_add = parts[4] == "+";
    let operation = move |a, b| if is_add { a + b } else { a * b };
    match parts[5] {
        "old" => Box::new(move |x| operation(x, x)),
        n => {
            let num = n.parse::<i32>().unwrap();
            Box::new(move |x| operation(x, num))
        }
    }
}

fn parse_test(line: &str, true_monkey: usize, false_monkey: usize) -> Box<dyn Fn(i32) -> usize> {
    let divisor = get_last_number(line);
    Box::new(move |x| if x % divisor == 0 { true_monkey } else { false_monkey })
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let monkeys = input.lines().chunks(7);
    monkeys
        .into_iter()
        .map(|monkey| {
            let monkey: Vec<&str> = monkey.collect();
            let true_monkey = get_last_number(monkey[4]) as usize;
            let false_monkey = get_last_number(monkey[5]) as usize;
            Monkey {
                items: parse_starting_items(monkey[1]),
                operation: parse_operation(monkey[2]),
                test: parse_test(monkey[3], true_monkey, false_monkey),
            }
        })
        .collect()
}

fn simulate(monkeys: &Vec<Monkey>, rounds: usize, divisor: i32) -> usize {
    let mut items: Vec<Vec<i32>> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut inspections = vec![0; monkeys.len()];
    
    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            inspections[i] += items[i].len();
            for item in items[i].drain(..).collect::<Vec<_>>() {
                let new_item = (monkey.operation)(item) / divisor;
                items[(monkey.test)(new_item)].push(new_item);
            }
        }
    }
    inspections.into_iter().sorted().rev().take(2).product()
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let monkeys = parse_monkeys(&input);
    println!("task1: {}", simulate(&monkeys, 20, 3));
    println!("task2: {}", simulate(&monkeys, 10_000, 1));
    Ok(())
}
