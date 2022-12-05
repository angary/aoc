use std::{collections::HashSet, error::Error, fs};

fn task(string: String, seq_size: usize) -> usize {
    string
        .as_bytes()
        .windows(seq_size)
        .enumerate()
        .find(|(_, w)| (HashSet::from_iter(w.iter().cloned()) as HashSet<u8>).len() == seq_size)
        .unwrap()
        .0
        + seq_size
}

fn task_1(string: String) -> usize {
    task(string, 4)
}

fn task_2(string: String) -> usize {
    task(string, 14)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("input.txt")?;
    println!("task 1: {}", task_1(data.clone()));
    println!("task 2: {}", task_2(data));
    Ok(())
}
