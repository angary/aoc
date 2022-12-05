use std::{error::Error, fs};

fn score(c: char) -> i32 {
    c as i32
        - if c.is_lowercase() {
            'a' as i32 - 1
        } else {
            'A' as i32 - 27
        }
}

fn task_1(lines: &[String]) -> i32 {
    lines
        .iter()
        .map(|s| {
            let (s1, s2) = s.split_at(s.len() / 2);
            score(s1.chars().find(|c| s2.contains(*c)).unwrap())
        })
        .sum()
}
fn task_2(lines: &[String]) -> i32 {
    lines
        .chunks(3)
        .map(|s| {
            score(
                s[0].chars()
                    .find(|c| s[1].contains(*c) && s[2].contains(*c))
                    .unwrap(),
            )
        })
        .sum()
}
fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("input.txt")?;
    let lines: Vec<String> = text.split('\n').map(String::from).collect();
    println!("task 1: {}", task_1(&lines));
    println!("task 2: {}", task_2(&lines));
    Ok(())
}
