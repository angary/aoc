use std::error::Error;
use itertools::Itertools;

fn line_of_sight_trees(trees: &[Vec<usize>], pos: (usize, usize)) -> Vec<Vec<usize>> {
    let (i, j) = pos;
    vec![
        trees[i][..j].iter().rev().copied().collect(),       // east
        trees[i][j + 1..].to_vec(),                          // west
        trees[..i].iter().rev().map(|row| row[j]).collect(), // north
        trees[i + 1..].iter().map(|row| row[j]).collect(),   // south
    ]
}

fn is_visible(trees: &[Vec<usize>], tree: &usize, pos: (usize, usize)) -> bool {
    line_of_sight_trees(trees, pos)
        .iter()
        .any(|directional_trees| match directional_trees.iter().max() {
            Some(height) => tree > height,
            _ => true,
        })
}

fn score(trees: &[Vec<usize>], tree: &usize, pos: (usize, usize)) -> usize {
    line_of_sight_trees(trees, pos)
        .iter()
        .map(|trees| trees.iter()
            .position(|&h| h >= *tree)
            .map_or(trees.len(), |i| i + 1))
        .product()
}

fn task_1(trees: &[Vec<usize>]) -> usize {
    (0..trees.len())
        .cartesian_product(0..trees[0].len())
        .filter(|&(i, j)| is_visible(trees, &trees[i][j], (i, j)))
        .count()
}

fn task_2(trees: &[Vec<usize>]) -> usize {
    (0..trees.len())
        .cartesian_product(0..trees[0].len())
        .map(|(i, j)| score(trees, &trees[i][j], (i, j)))
        .max()
        .unwrap()
}

fn parse_trees(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let trees = parse_trees(&input);
    println!("task 1: {}", task_1(&trees));
    println!("task 2: {}", task_2(&trees));
    Ok(())
}
