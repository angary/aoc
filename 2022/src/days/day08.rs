use std::error::Error;
use itertools::Itertools;

type Position = (usize, usize);
type Direction = Vec<usize>;

#[derive(Debug)]
struct TreeGrid {
    grid: Vec<Vec<usize>>,
}

impl TreeGrid {
    fn new(grid: Vec<Vec<usize>>) -> Self {
        Self { grid }
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    /// Returns trees visible in each cardinal direction from a position.
    /// Order: east, west, north, south
    fn line_of_sight(&self, pos: Position) -> Vec<Direction> {
        let (i, j) = pos;
        vec![
            self.grid[i][..j].iter().rev().copied().collect(), // east
            self.grid[i][j + 1..].to_vec(),                    // west
            self.grid[..i].iter().rev().map(|row| row[j]).collect(), // north
            self.grid[i + 1..].iter().map(|row| row[j]).collect(), // south
        ]
    }

    /// Checks if a tree is visible from any direction
    fn is_visible(&self, pos: Position) -> bool {
        let (i, j) = pos;
        let tree_height = self.grid[i][j];
        
        // Edge trees are always visible
        if i == 0 || i == self.height() - 1 || j == 0 || j == self.width() - 1 {
            return true;
        }

        self.line_of_sight(pos)
            .iter()
            .any(|trees| trees.iter().all(|&h| h < tree_height))
    }

    /// Calculates scenic score for a tree
    fn scenic_score(&self, pos: Position) -> usize {
        let (i, j) = pos;
        let tree_height = self.grid[i][j];
        
        self.line_of_sight(pos)
            .iter()
            .map(|trees| calculate_viewing_distance(trees, tree_height))
            .product()
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidDigit(char),
    EmptyInput,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidDigit(c) => write!(f, "invalid tree height digit: {}", c),
            ParseError::EmptyInput => write!(f, "input is empty"),
        }
    }
}

impl Error for ParseError {}

/// Calculates how far a tree can see in one direction
fn calculate_viewing_distance(trees: &[usize], height: usize) -> usize {
    trees
        .iter()
        .position(|&h| h >= height)
        .map_or(trees.len(), |i| i + 1)
}

/// Parses input into a grid of tree heights
fn parse_trees(input: &str) -> Result<TreeGrid, ParseError> {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as usize)
                        .ok_or(ParseError::InvalidDigit(c))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    if grid.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    Ok(TreeGrid::new(grid))
}

/// Counts visible trees from outside the grid
fn task_1(trees: &TreeGrid) -> usize {
    (0..trees.height())
        .cartesian_product(0..trees.width())
        .filter(|&pos| trees.is_visible(pos))
        .count()
}

/// Finds the highest scenic score possible
fn task_2(trees: &TreeGrid) -> usize {
    (0..trees.height())
        .cartesian_product(0..trees.width())
        .map(|pos| trees.scenic_score(pos))
        .max()
        .unwrap_or(0)
}

pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    let trees = parse_trees(&input)?;
    println!("task 1: {}", task_1(&trees));
    println!("task 2: {}", task_2(&trees));
    Ok(())
}
