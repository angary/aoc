use std::{cmp::Reverse, error::Error};

fn task_1(calories: &[u32]) -> u32 {
    *calories.first().expect("Missing first calorie")
}

fn task_2(calories: &[u32]) -> u32 {
    calories.iter().take(3).sum()
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect();
    calories.sort_by_key(|c| Reverse(*c));
    println!("task 1: {}", task_1(&calories));
    println!("task 2: {}", task_2(&calories));
    Ok(())
}
