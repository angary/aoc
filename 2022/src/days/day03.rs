use std::error::Error;

fn score(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => 0, // Handle unexpected characters
    }
}

fn task_1(lines: &[String]) -> i32 {
    lines
        .iter()
        .filter_map(|s| {
            let (s1, s2) = s.split_at(s.len() / 2);
            s1.chars().find(|c| s2.contains(*c)).map(score)
        })
        .sum()
}

fn task_2(lines: &[String]) -> i32 {
    lines
        .chunks(3)
        .filter_map(|s| {
            s[0].chars()
                .find(|c| s[1].contains(*c) && s[2].contains(*c))
                .map(score)
        })
        .sum()
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = input.split('\n').map(String::from).collect();
    println!("task 1: {}", task_1(&lines));
    println!("task 2: {}", task_2(&lines));
    Ok(())
}
