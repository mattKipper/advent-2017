use std::collections::HashSet;
use std::env::args;
use std::process::exit;

/// Checks if a string slice is a valid passphrase (i.e. no repeated words)
fn is_valid(passphrase: &str) -> bool {
    let mut word_map = HashSet::new();
    passphrase.split_whitespace().all(|word| word_map.insert(word))
}

fn valid_passphrases(input: String) -> usize {
    input.lines().filter(|line| is_valid(line)).count()
}

fn print_usage() {
    println!("Day 4: High Entropy Passphrase");
    println!("Usage:");
    println!("04 <input>");
    println!("  <inputs> - One or more passphrases (newline separated)");
}

fn main() {
    if let (2, Some(input)) = (args().count(), args().nth(1)) {
        println!("{}", valid_passphrases(input));
    }
    else {
        print_usage();
        exit(-1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_passphrase() {
        assert!(is_valid(" abc de f g abcd"));
    }

    #[test]
    fn invalid_passphrase() {
        assert!(!is_valid(" abc de f   g    abc"));
    }

    #[test]
    fn multiple_valid() {
        let input = String::from("ab cd \n ab ab \n ab cd \n ab cd");
        assert_eq!(valid_passphrases(input), 3);
    }

    #[test]
    fn no_valid() {
        let input = String::from("ab ab\nab ab\nab ab\nab ab");
        assert_eq!(valid_passphrases(input), 0);
    }
}