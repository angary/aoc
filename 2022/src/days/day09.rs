use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use super::lib::Point;

static POINT_MAP: Lazy<HashMap<&str, Point>> = Lazy::new(|| {
    HashMap::from([
        ("U", Point::new(0, 1)),
        ("R", Point::new(1, 0)),
        ("D", Point::new(0, -1)),
        ("L", Point::new(-1, 0)),
    ])
});

const NUM_KNOTS: usize = 10;

fn should_move_tail(head: Point, tail: Point) -> bool {
    let dx = (head.x - tail.x).abs();
    let dy = (head.y - tail.y).abs();
    (dx + dy > 2) || (dx == 2 && dy == 0) || (dy == 2 && dx == 0)
}

fn task_1(directions: &[(Point, i32)]) -> usize {
    let mut head = Point::new(0, 0);
    let mut tail = head.clone();
    let mut visited: HashSet<Point> = HashSet::from([tail]);
    for (direction, scale) in directions {
        for _ in 0..*scale {
            head += *direction;
            if should_move_tail(head, tail) {
                tail = head - *direction;
                visited.insert(tail);
            }
        }
    }
    visited.len()
}

fn task_2(directions: &[(Point, i32)]) -> usize {
    let mut visited: HashSet<Point> = HashSet::from([Point::new(0, 0)]);
    let mut string = vec![Point::new(0, 0); NUM_KNOTS];
    for (direction, scale) in directions {
        for _ in 0..*scale {
            // Move the head
            string[0] += *direction;
            let mut curr_direction = *direction;
            for index in 0..NUM_KNOTS - 1 {
                let head = string[index];
                let tail = string[index + 1];
                let dx = (head.x - tail.x).abs();
                let dy = (head.y - tail.y).abs();
                // We do a diagonal jump
                if (dx + dy > 2) {
                    let new_tail = head - curr_direction;
                    string[index + 1] = new_tail;
                    curr_direction = new_tail - tail;
                }
                
                
                (dx == 2 && dy == 0) || (dy == 2 && dx == 0)
                if should_move_tail(head, tail) {
                    let new_tail = head - curr_direction;
                    string[index + 1] = new_tail;
                    curr_direction = new_tail - tail;
                } else {
                    break;
                }
            }
            println!(
                "{:?}",
                string
                    .iter()
                    .map(|p| (p.x, p.y))
                    .collect::<Vec<(i32, i32)>>()
            );
            visited.insert(*string.last().unwrap());
        }
        println!();
        // println!("{:?}", visited);
    }
    visited.len()
}

fn parse_directions(input: String) -> Vec<(Point, i32)> {
    input
        .lines()
        .map(|line| {
            let (direction, scale) = line
                .split_once(' ')
                .expect("input should be in format <direction> <scale>");
            let direction = POINT_MAP.get(&direction).expect("direction is one of URDL");
            let scale: i32 = scale.parse().expect("scale is a number");
            (*direction, scale)
        })
        .collect()
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let directions = parse_directions(input);
    println!("task 1: {}", task_1(&directions));
    println!("task 2: {}", task_2(&directions));
    Ok(())
}
