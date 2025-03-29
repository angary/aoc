use std::{collections::HashSet, error::Error, fmt};

use super::lib::Point;

/// Represents possible errors that can occur during parsing
#[derive(Debug)]
enum RopeError {
    InvalidDirection(String),
    InvalidScale(String),
    InvalidFormat(String),
}

impl fmt::Display for RopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RopeError::InvalidDirection(dir) => write!(f, "Invalid direction: {}", dir),
            RopeError::InvalidScale(scale) => write!(f, "Invalid scale: {}", scale),
            RopeError::InvalidFormat(line) => write!(f, "Invalid line format: {}", line),
        }
    }
}

impl Error for RopeError {}

/// Returns the mapping of direction characters to their corresponding Point vectors
fn get_direction(dir: &str) -> Option<Point> {
    match dir {
        "U" => Some(Point::new(0, 1)),
        "R" => Some(Point::new(1, 0)),
        "D" => Some(Point::new(0, -1)),
        "L" => Some(Point::new(-1, 0)),
        _ => None,
    }
}

/// Parses a single line of input into a direction and scale
fn parse_line(line: &str) -> Result<(Point, i32), RopeError> {
    let (dir, scale) = line.split_once(' ')
        .ok_or_else(|| RopeError::InvalidFormat(line.to_string()))?;
    
    let direction = get_direction(dir)
        .ok_or_else(|| RopeError::InvalidDirection(dir.to_string()))?;
    
    let scale = scale.parse()
        .map_err(|_| RopeError::InvalidScale(scale.to_string()))?;
    
    Ok((direction, scale))
}

/// Parses the input string into a vector of directions and scales
fn parse_directions(input: String) -> Result<Vec<(Point, i32)>, RopeError> {
    input.lines().map(parse_line).collect()
}

/// Updates the position of a tail knot based on its head knot's position
fn update_knot_position(head: Point, tail: &mut Point) {
    let delta = head - *tail;
    if delta.x.abs() > 1 || delta.y.abs() > 1 {
        tail.x += delta.x.signum();
        tail.y += delta.y.signum();
    }
}

/// Simulates rope movement and returns the number of unique positions visited by the tail
/// 
/// # Arguments
/// * `directions` - List of movement directions and their scales
/// * `length` - Number of knots in the rope (must be >= 2)
/// 
/// # Panics
/// * If length < 2 (rope must have at least a head and tail)
fn simulate(directions: &[(Point, i32)], length: usize) -> usize {
    assert!(length >= 2, "Rope must have at least 2 knots");
    
    let mut rope = vec![Point::new(0, 0); length];
    let mut visited = HashSet::from([Point::new(0, 0)]);

    for &(direction, scale) in directions {
        for _ in 0..scale {
            rope[0] += direction;
            for i in 1..length {
                update_knot_position(rope[i - 1], &mut rope[i]);
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
    let directions = parse_directions(input)?;
    println!("task 1: {}", task_1(&directions));
    println!("task 2: {}", task_2(&directions));
    Ok(())
}
