use std::collections::HashSet;
use std::env::args;
use std::process::exit;

fn max_bank(banks: &Vec<u32>) -> (usize, u32) {
    banks.iter()
         .enumerate()
         .fold((0,0), |(max_i, max_v), (i, &v)| {
            if v > max_v {
                (i, v)
            }
            else {
                (max_i, max_v)
            }
            })
}

fn redistribute(banks: &mut Vec<u32>) {

    let (max_index, max_value) = max_bank(&banks);
    let len = banks.len();

    // Clear out the bank being redistributed
    banks[max_index] = 0;

    // All banks will add at least (max_value / len) blocks.
    for val in banks.iter_mut() {
        *val += max_value / len as u32;
    }

    // After the initial distribution of (max_value / len) blocks,
    // it's guaranteed that the remaining blocks, (max_value % len),
    // will be less than len. However, these blocks are distributed
    // starting at the bank following banks[max_index] and may wrap
    // back to the start. There's probably a graceful way to do this,
    // but the simplest way is to iterate from the first bank after 
    // banks[max_index], decrementing the remaining blocks and cycling
    // back to the start if necessary.
    let mut remaining_blocks = max_value % len as u32;
    for val in banks.iter_mut().skip(max_index + 1) {
        if remaining_blocks > 0 {
            *val += 1;
            remaining_blocks -= 1;
        }
        else {
            break;
        }
    }

    // Handle the overflow case
    for val in banks.iter_mut() {
        if remaining_blocks > 0 {
            *val += 1;
            remaining_blocks -= 1;
        }
        else {
            break;
        }
    }
}

fn cycle_count(mut banks: Vec<u32>) -> u32 {
    let mut distributions = HashSet::new(); 
    while distributions.insert(banks.clone()) {
        redistribute(&mut banks);
    }
    distributions.len() as u32
}

fn print_usage() {
    println!("Day 6: Memory Reallocation");
    println!("Usage:");
    println!("06 <input>");
    println!("  <input> - Memory banks (whitespace-separated)");
}

fn main() {
    if let (2, Some(input)) = (args().len(), args().nth(1)) {
        match input.split_whitespace()
                   .map(|x| x.parse::<u32>())
                   .collect::<Result<Vec<u32>, _>>() {
            Ok(input) => println!("{}", cycle_count(input)),
            Err(_) => {
                println!("Error: Input must be unsigned integers.");
                exit(-1);
            }
        }
    }
    else {
        print_usage();
        exit(-1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example_max() {
        assert_eq!(max_bank(&vec![1,3,3,2]), (1, 3));
    }

    #[test]
    fn aoc_example_redistributions() {
        let mut input = vec![0,2,7,0];

        redistribute(&mut input);
        assert_eq!(input, vec![2,4,1,2]);

        redistribute(&mut input);
        assert_eq!(input, vec![3,1,2,3]);

        redistribute(&mut input);
        assert_eq!(input, vec![0,2,3,4]);

        redistribute(&mut input);
        assert_eq!(input, vec![1,3,4,1]);

        redistribute(&mut input);
        assert_eq!(input, vec![2,4,1,2]);
    }

    #[test]
    fn aoc_example_cycle_count() {
        assert_eq!(cycle_count(vec![0,2,7,0]), 5);
    }
}