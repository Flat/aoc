use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() -> Result<(), Box<Error>> {
    let matches = App::new("aoc-1-1")
        .about("Solves Advent of Code day 1 part 1")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file from aoc-1-1.")
            .required(true)
            .index(1))
        .get_matches();

    let input = matches.value_of("INPUT").ok_or("Input not found!")?;
    let input_file = File::open(input)?;
    let mut input_vec = Vec::new();
    for line in BufReader::new(input_file).lines() {
        if let Ok(num) = line {
            let num = num.replace('+', "");
            input_vec.push(num.parse::<i32>()?)
        }
    }
    let mut count = 0;
    for num in input_vec {
        count += num;
    }
    Ok(println!("{}", count))
}
