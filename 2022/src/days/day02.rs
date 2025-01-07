use std::error::Error;

fn task_1(s: &String) -> i32 {
    let a = s.chars().next().unwrap() as i32 - 'A' as i32; // 0 = rock, 1 = paper, 2 = scissors
    let b = s.chars().nth(2).unwrap() as i32 - 'X' as i32;
    let win = (b - a + 4) % 3;
    (win * 3) + (b + 1)
}

fn task_2(s: &String) -> i32 {
    let a = s.chars().next().unwrap() as i32 - 'A' as i32; // 0 = rock, 1 = paper, 2 = scissors
    let win = s.chars().nth(2).unwrap() as i32 - 'X' as i32;
    let b = (win + a + 2) % 3; // inverse func of above
    (win * 3) + (b + 1)
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let data: Vec<String> = input.split("\n").map(String::from).collect();
    println!("task 1: {}", data.iter().map(task_1).sum::<i32>());
    println!("task 2: {}", data.iter().map(task_2).sum::<i32>());
    Ok(())
}
