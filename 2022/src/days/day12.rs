use std::{collections::HashSet, error::Error};

use itertools::Itertools;

type ParsedInput = (Vec<Vec<char>>, (usize, usize), (usize, usize));
fn get_2d_index(index: usize, map: &[Vec<char>]) -> (usize, usize) {
    let width = map[0].len() + 1;
    (index / width, index % width)
}

fn parse_input(input: String) -> ParsedInput {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = get_2d_index(input.find('S').unwrap(), &map);
    let end = get_2d_index(input.find('E').unwrap(), &map);
    map[start.0][start.1] = 'a';
    map[end.0][end.1] = 'z';
    (map, start, end)
}

fn get_neighbors(
    map: &[Vec<char>],
    visited: &HashSet<(usize, usize)>,
    (y, x): (usize, usize),
) -> Vec<(usize, usize)> {
    [
        (y.wrapping_sub(1), x),
        (y + 1, x),
        (y, x.wrapping_sub(1)),
        (y, x + 1),
    ]
    .into_iter()
    .filter(|&(ny, nx)| {
        ny < map.len()
            && nx < map[0].len()
            && map[ny][nx] as u32 <= map[y][x] as u32 + 1
            && !visited.contains(&(ny, nx))
    })
    .collect()
}

fn bfs(map: &[Vec<char>], start: Vec<(usize, usize)>, end: (usize, usize)) -> usize {
    let mut distance = 0;
    let mut queue = start.clone();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while !queue.is_empty() {
        if queue.iter().any(|point| *point == end) {
            return distance;
        }
        visited.extend(queue.iter());
        queue = queue
            .into_iter()
            .flat_map(|point| get_neighbors(map, &visited, point))
            .unique()
            .collect();
        distance += 1;
    }
    distance
}

fn task_1(map: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> usize {
    bfs(map, vec![start], end)
}

fn task_2(map: &[Vec<char>], end: (usize, usize)) -> usize {
    let mut starts = vec![];
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'a' {
                starts.push((i, j));
            }
        }
    }
    bfs(map, starts, end)
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let (map, start, end) = parse_input(input);
    println!("task 1: {}", task_1(&map, start, end));
    println!("task 2: {}", task_2(&map, end));
    Ok(())
}
