use std::{error::Error, fs};

type TwoRanges = (u32, u32, u32, u32);

fn task_1((a0, a1, b0, b1): &&TwoRanges) -> bool {
    (a0 <= b0 && b1 <= a1) || (b0 <= a0 && a1 <= b1)
}

fn task_2((a0, a1, b0, b1): &&TwoRanges) -> bool {
    u32::min(*a1, *b1) >= u32::max(*a0, *b0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("input.txt")?;
    let data: Vec<TwoRanges> = text
        .split('\n')
        .map(|s| {
            let x: Vec<u32> = s
                .split(&[',', '-'])
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            (x[0], x[1], x[2], x[3])
        })
        .collect();
    println!("task 1: {}", data.iter().filter(task_1).count());
    println!("task 2: {}", data.iter().filter(task_2).count());
    Ok(())
}
