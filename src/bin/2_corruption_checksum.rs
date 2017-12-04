use std::num;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn print_usage() {
    println!("Day 2: Corruption Checksum");
    println!("Usage:");
    println!("  2_corruption_checksum <input_file>");
    println!("  <input_file> - Tab-separated spreadsheet");
}

#[derive(Debug)]
enum InputError {
    Usage,
    Io(io::Error),
    Parse(num::ParseIntError),
}

type Input = Vec<Vec<u32>>;
fn get_input() -> Result<Input, InputError> {
    match std::env::args().nth(1) {
        None => Err(InputError::Usage),
        Some(filename) => input_from_file(filename),
    }
}

fn input_from_file(filename: String) -> Result<Input, InputError> {

    match File::open(filename) {

        Ok(mut file) => {

            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => parse_input(contents),
                Err(err) => Err(InputError::Io(err))
            }
        },

        Err(err) => Err(InputError::Io(err))
    }
}

fn parse_input(text: String) -> Result<Input, InputError> {

    let mut values: Input = Vec::new();

    for line in text.lines() {
        let mut row: Vec<u32> = Vec::new();
        for entry in line.trim().split('\t') {
            match entry.parse::<u32>() {
                Ok(val) => row.push(val),
                Err(err) => return Err(InputError::Parse(err)),
            };
        }
        values.push(row);
    }

    Ok(values)
}

fn checksum(input: Input) -> u32 {
    0
}

fn main() {

    let input = get_input();

    match input {
        Ok(input) => println!("{}", checksum(input)),

        Err(InputError::Usage) => {
            print_usage();
            std::process::exit(-1);
        }

        Err(InputError::Io(err)) => {
            println!("IO Error: {:?}", err);
            std::process::exit(-1);
        }

        Err(InputError::Parse(err)) => {
            println!("Parsing Error: {:?}", err);
            std::process::exit(-1);
        }
    }
}
