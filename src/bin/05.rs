use std::env::args;
use std::num;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;


/// Performs a single jump instruction. Returns Some() with the next
/// index if the new index is inside the list of jumps, or None if
/// it's out-of-bounds.
fn take_jump(jumps: &mut Vec<i32>, index: usize) -> Option<usize> {
    let new_index = (index as i32) + jumps[index];

    if new_index >= 0 && (new_index as usize) < jumps.len() {
        jumps[index] += 1;
        Some(new_index as usize)
    } else {
        None
    }
}

/// Counts the number of jumps needed to break from the list
fn count_jumps(mut jumps: Vec<i32>) -> u32 {
    let (mut count, mut index) = (0, 0);
    while let Some(next_index) = take_jump(&mut jumps, index) {
        count += 1;
        index = next_index;
    }
    count + 1 // Last jump isn't counted in the loop
}

fn print_usage() {
    println!("Day 5: High Entropy Passphrase");
    println!("Usage:");
    println!("05 <input_file>");
    println!("  <input_file> - Jump input file (newline-separated");
}

#[derive(Debug)]
enum InputError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

/// Generates a vector of jump values from a file (via filename)
fn jumps_from_file(filename: String) -> Result<Vec<i32>, InputError> {
    match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();

            match file.read_to_string(&mut contents) {
                Ok(_) => match contents.lines().map(|x| x.parse::<i32>()).collect() {
                    Ok(jumps) => Ok(jumps),
                    Err(e) => Err(InputError::Parse(e)),
                },
                Err(e) => Err(InputError::Io(e)),
            }
        }

        Err(e) => Err(InputError::Io(e)),
    }
}

fn main() {
    // Usage is one extra arg (input filename)
    if let (2, Some(filename)) = (args().count(), args().nth(1)) {
        match jumps_from_file(filename) {
            Ok(jumps) => println!("{}", count_jumps(jumps)),

            Err(InputError::Io(e)) => {
                println!("IO Error: {}", e);
                exit(-1);
            }

            Err(InputError::Parse(e)) => {
                println!("Parsing Error: {}", e);
                exit(-1);
            }
        }
    } else {
        print_usage();
        exit(-1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_step() {
        let mut input = vec![1, 2, 0, -1];
        assert_eq!(take_jump(&mut input, 1), Some(3));
        assert_eq!(input, vec![1, 3, 0, -1]);
    }

    #[test]
    fn single_step_out() {
        let mut input = vec![1, 3, 0, -1];
        let original = input.clone();
        assert_eq!(take_jump(&mut input, 1), None);
        assert_eq!(input, original);
    }

    #[test]
    fn aoc_example_first_step() {
        let mut input = vec![0, 3, 0, 1, -3];
        assert_eq!(take_jump(&mut input, 0), Some(0));
        assert_eq!(input, vec![1, 3, 0, 1, -3]);
    }

    #[test]
    fn aoc_example() {
        assert_eq!(count_jumps(vec![0, 3, 0, 1, -3]), 5);
    }
}
