use std::{cmp::Reverse, error::Error};

fn task_1(calories: &[u32]) -> u32 {
    calories[0]
}

fn task_2(calories: &[u32]) -> u32 {
    calories.iter().take(3).sum::<u32>()
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let mut calories = input
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .map(|v| v.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    calories.sort_by_key(|c| Reverse(*c));
    println!("task 1: {}", task_1(&calories));
    println!("task 2: {}", task_2(&calories));
    Ok(())
}
