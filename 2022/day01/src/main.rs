use std::{cmp::Reverse, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("input.txt")?;
    let mut calories = text
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|v| v.parse::<u32>().expect("Input should be an int"))
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    calories.sort_by_key(|c| Reverse(*c));
    println!("{}", calories.iter().take(3).sum::<u32>());
    Ok(())
}
