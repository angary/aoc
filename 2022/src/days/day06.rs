use std::{collections::HashSet, error::Error};

fn task(string: &str, seq_size: usize) -> Option<usize> {
    string
        .as_bytes()
        .windows(seq_size)
        .enumerate()
        .find(|(_, w)| (HashSet::from_iter(w.iter().cloned()) as HashSet<u8>).len() == seq_size)
        .map(|(index, _)| index + seq_size)
}

fn task_1(string: &str) -> usize {
    task(string, 4).expect("No valid window found for task 1")
}

fn task_2(string: &str) -> usize {
    task(string, 14).expect("No valid window found for task 2")
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    println!("task 1: {}", task_1(&input));
    println!("task 2: {}", task_2(&input));
    Ok(())
}
