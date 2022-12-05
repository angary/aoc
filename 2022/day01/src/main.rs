use std::{cmp::Reverse, error::Error, fs};
fn task_1(calories: &[u32]) -> u32 {
    calories[0]
}
fn task_2(calories: &[u32]) -> u32 {
    calories.iter().take(3).sum::<u32>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("input.txt")?;
    let mut calories = text
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
