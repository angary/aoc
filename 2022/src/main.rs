use clap::{command, Parser};
use std::{error::Error, fs};
pub mod days;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    day: String,

    #[arg(index = 2, default_value = "input.txt")]
    input: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let padded_day = format!("{:0>2}", args.day);
    let text = fs::read_to_string(args.input)?;
    match padded_day.as_str() {
        "01" => days::day01::main(text),
        "02" => days::day02::main(text),
        "03" => days::day03::main(text),
        "04" => days::day04::main(text),
        "05" => days::day05::main(text),
        "06" => days::day06::main(text),
        "07" => days::day07::main(text),
        "08" => days::day08::main(text),
        "09" => days::day09::main(text),
        "10" => days::day10::main(text),
        _ => Err(format!("Couldn't find the day: {}", args.day).into())
    }
}
