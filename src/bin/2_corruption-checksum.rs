use std::num;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::cmp;

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

/// Calculate the checksum, provided a valid input matrix. 
fn checksum(input: Input) -> u32 {
    // The following approach is used:
    //  1. map() each row from Vec<u32> -> (min: u32, max: u32)
    //  2. Take the sum of (max - min) for all (min, max) tuples
    input.iter().map( |row| {
        row.iter().fold( 
            (row.first().unwrap(), row.first().unwrap()), 
            |(min, max), x| {
                (cmp::min(min, x), cmp::max(max, x))
            }
        )
    })
    .fold(0, |acc, (min, max)| acc + max - min)
}

/// Calculates the "checksum" of an input matrix file, printing the
/// result to stdout. 
///
/// The input file has the following format:
///     1. Columns tab-separated (\t)
///     2. Rows newline-separated (\n)
///     3. Values are unsigned 32-bit integers
/// 
/// The checksum is defined as sum(max(row) - min(row)) for all rows. 
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_error_non_numeric() {
        assert!(parse_input(String::from("1\t2\t3\n4\ta\t5\n")).is_err());
    }

    #[test]
    fn parse_error_negative() {
        assert!(parse_input(String::from("1\t2\t3\n4\t5\t-2\n")).is_err());
    }

    #[test]
    fn parse_success() {
        let input = String::from("1\t2\t3\n9\t8\t7\n");
        assert_eq!(
            parse_input(input).unwrap(),
            vec![ vec![1, 2, 3], vec![9, 8, 7]]
        );
    }

    #[test]
    fn checksum_nonzero() {
        let input = vec![ vec![6, 5, 1], vec![3, 5, 8], vec![5, 1, 3]];
        assert_eq!(checksum(input), 14);
    }

    #[test]
    fn checksum_zero() {
        let input = vec![ vec![1, 1], vec![2, 2], vec![99, 99]];
        assert_eq!(checksum(input), 0);
    }
}
