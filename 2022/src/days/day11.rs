use std::error::Error;

use itertools::Itertools;
use regex::Regex;

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> usize>,
}

fn parse_starting_items(line: &str) -> Vec<i64> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(line)
        .flat_map(|m| m.as_str().parse())
        .collect()
}

fn get_last_number(line: &str) -> i64 {
    line.split_whitespace().last().unwrap().parse().unwrap()
}

fn parse_operation(line: &str) -> Box<dyn Fn(i64) -> i64> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let is_add = parts[4] == "+";
    let operation = move |a, b| if is_add { a + b } else { a * b };
    match parts[5] {
        "old" => Box::new(move |x| operation(x, x)),
        n => {
            let num = n.parse::<i64>().unwrap();
            Box::new(move |x| operation(x, num))
        }
    }
}

fn parse_test(divisor: i64, true_monkey: usize, false_monkey: usize) -> Box<dyn Fn(i64) -> usize> {
    Box::new(move |x| {
        if x % divisor == 0 {
            true_monkey
        } else {
            false_monkey
        }
    })
}

fn parse_monkeys(input: &str) -> (Vec<Monkey>, i64) {
    let monkeys = input.lines().chunks(7);
    let mut lcm = 1;
    let monkeys = monkeys
        .into_iter()
        .map(|monkey| {
            let monkey: Vec<&str> = monkey.collect();
            let divisor = get_last_number(monkey[3]);
            lcm *= divisor;
            let true_monkey = get_last_number(monkey[4]) as usize;
            let false_monkey = get_last_number(monkey[5]) as usize;
            Monkey {
                items: parse_starting_items(monkey[1]),
                operation: parse_operation(monkey[2]),
                test: parse_test(divisor, true_monkey, false_monkey),
            }
        })
        .collect();
    (monkeys, lcm)
}

fn simulate(monkeys: &[Monkey], lcm: i64, rounds: usize, divisor: i64) -> usize {
    let mut items: Vec<Vec<i64>> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            inspections[i] += items[i].len();
            for item in items[i].drain(..).collect::<Vec<_>>() {
                let new_item = (monkey.operation)(item) / divisor;
                items[(monkey.test)(new_item)].push(new_item % lcm);
            }
        }
    }
    inspections.into_iter().sorted().rev().take(2).product()
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let (monkeys, lcm) = parse_monkeys(&input);
    println!("task 1: {}", simulate(&monkeys, lcm, 20, 3));
    println!("task 2: {}", simulate(&monkeys, lcm, 10_000, 1));
    Ok(())
}
