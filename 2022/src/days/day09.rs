use std::{collections::HashSet, error::Error};

use super::lib::Point;

fn parse_directions(input: String) -> Vec<(Point, i32)> {
    input
        .lines()
        .map(|line| {
            let (c, scale) = line.split_once(' ').unwrap();
            let direction = match c {
                "U" => Point::new(0, 1),
                "R" => Point::new(1, 0),
                "D" => Point::new(0, -1),
                "L" => Point::new(-1, 0),
                _ => panic!("Invalid direction: {}", c),
            };
            (direction, scale.parse().unwrap())
        })
        .collect()
}

fn simulate(directions: &[(Point, i32)], length: usize) -> usize {
    let mut rope = vec![Point::new(0, 0); length];
    let mut visited = HashSet::from([Point::new(0, 0)]);
    for (direction, scale) in directions {
        for _ in 0..*scale {
            rope[0] += *direction;
            for i in 1..length {
                let delta = rope[i - 1] - rope[i];
                if delta.x.abs() > 1 || delta.y.abs() > 1 {
                    rope[i].x += delta.x.signum();
                    rope[i].y += delta.y.signum();
                }
            }
            visited.insert(rope[length - 1]);
        }
    }
    visited.len()
}

fn task_1(directions: &[(Point, i32)]) -> usize {
    simulate(directions, 2)
}

fn task_2(directions: &[(Point, i32)]) -> usize {
    simulate(directions, 10)
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let directions = parse_directions(input);
    println!("task 1: {}", task_1(&directions));
    println!("task 2: {}", task_2(&directions));
    Ok(())
}
