use std::error::Error;

fn calculate_score(win: i32, player: i32) -> i32 {
    (win * 3) + (player + 1)
}

fn task_1(s: &str) -> i32 {
    let player = s.chars().next().unwrap() as i32 - 'A' as i32; // 0 = rock, 1 = paper, 2 = scissors
    let opponent = s.chars().nth(2).unwrap() as i32 - 'X' as i32;
    let win = (player - opponent + 4) % 3;
    calculate_score(win, player)
}

fn task_2(s: &str) -> i32 {
    let opponent = s.chars().next().unwrap() as i32 - 'A' as i32; // 0 = rock, 1 = paper, 2 = scissors
    let win = s.chars().nth(2).unwrap() as i32 - 'X' as i32;
    let player = (win + opponent + 2) % 3; // inverse func of above
    calculate_score(win, player)
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let data: Vec<&str> = input.lines().collect();
    let task1_result: i32 = data.iter().map(|s| task_1(s)).sum();
    let task2_result: i32 = data.iter().map(|s| task_2(s)).sum();
    println!("task 1: {}", task1_result);
    println!("task 2: {}", task2_result);
    Ok(())
}
