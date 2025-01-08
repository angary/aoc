use std::error::Error;

/// Returns a list of trees visible in each cardinal direction from the given position,
/// ordered by the order they are viewed from the original position.
fn line_of_sight_trees(trees: &[Vec<usize>], pos: (usize, usize)) -> Vec<Vec<usize>> {
    let (i, j) = pos;
    vec![
        trees[i][..j].iter().rev().copied().collect(), // east
        trees[i][j + 1..].to_vec(),                    // west
        trees[..i].iter().rev().map(|row| row[j]).collect(), // north
        trees[i + 1..].iter().map(|row| row[j]).collect(), // south
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
        .map(|directional_trees| {
            directional_trees
                .iter()
                .enumerate()
                .find(|(_, &other_tree)| other_tree >= *tree)
                .map(|(i, _)| i + 1)
                .unwrap_or(directional_trees.len())
        })
        .product()
}

fn task_1(trees: &[Vec<usize>]) -> usize {
    trees
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, &tree)| is_visible(trees, &tree, (i, *j)))
                .count()
        })
        .sum()
}

fn task_2(trees: &[Vec<usize>]) -> usize {
    trees
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &tree)| score(trees, &tree, (i, j)))
                .max()
                .unwrap()
        })
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
