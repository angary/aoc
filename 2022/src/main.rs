use std::{env, error::Error, fs};
pub mod days;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).expect("Missing day argument");
    let padded_day = format!("{:0>2}", day);
    let text = fs::read_to_string("input.txt")?;
    match padded_day.as_str() {
        "01" => days::day01::main(text),
        "02" => days::day02::main(text),
        "03" => days::day03::main(text),
        "04" => days::day04::main(text),
        "05" => days::day05::main(text),
        "06" => days::day06::main(text),
        _ => Err(format!("Couldn't find the day: {}", day).into())
    }
}
