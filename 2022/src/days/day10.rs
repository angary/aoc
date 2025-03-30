use std::error::Error;

use itertools::Itertools;

const CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];
const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;
const SPRITE_WIDTH: i32 = 3;

fn parse_instructions(input: String) -> Vec<i32> {
    input
        .lines()
        .flat_map(|line| match line {
            "noop" => vec![0],
            _ => vec![0, line[5..].parse().unwrap()], // else it is "addx X"
        })
        .collect()
}

fn task_1(instructions: &[i32]) -> i32 {
    let mut x = 1;
    let mut signal_strength = 0;
    for (cycle, instruction) in instructions.iter().enumerate() {
        let cycle = cycle as i32 + 1; // convert to 1-based indexing
        if CYCLES.contains(&cycle) {
            signal_strength += x * cycle;
        }
        x += instruction;
    }
    signal_strength
}

fn task_2(instructions: &[i32]) -> String {
    let mut screen = vec![vec!['.'; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut x: i32 = 0;
    for (i, row) in instructions.chunks(SCREEN_WIDTH).enumerate() {
        for (j, instruction) in row.iter().enumerate() {
            if (x..x + SPRITE_WIDTH).contains(&(j as i32)) {
                screen[i][j] = '#';
            }
            x += instruction;
        }
    }
    screen.iter().map(|row| row.iter().join("")).join("\n")
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let instructions = parse_instructions(input);
    println!("task 1: {}", task_1(&instructions));
    println!("task 2:\n{}", task_2(&instructions));
    Ok(())
}
