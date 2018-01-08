#![feature(test)]

extern crate rand;
extern crate test;

fn print_usage() {
    println!("Day 1: Inverse Captcha");
    println!("Usage:");
    println!("01 [-h / --help] <input_string>");
}

/// Parses arguments and returns the input string.
/// If the input string cannot be parsed, this returns
/// the desired exit code through Err(N).
fn get_input() -> Result<String, i32> {
    // cmdline should take exactly one argument
    match std::env::args().nth(1) {
        None => {
            print_usage();
            Err(-1)
        }
        Some(arg) => if arg == "-h" || arg == "--help" {
            print_usage();
            Err(0)
        } else {
            Ok(arg)
        },
    }
}

/// From an input string, return the sum of all sequential digit characters
/// in the list. The input string is circular, so the last character is
/// checked against the first chacter.
///
/// Non-digit characters are skipped but are considered separators
/// e.g.
/// `assert_eq!(calculate_sum(&String::from("a22")), 2);`
/// `assert_eq!(calculate_sum(&String::from("2a2")), 0);`
fn calculate_sum(s: &String) -> u32 {
    s.chars().zip(s.chars().skip(1))    // Zip pairs of consecutive chars
        .chain(s.chars().zip(s.chars().rev().take(1))) // Append (last,first)
        .filter(|&(first, second)| {    // Only including matching digits
            first.is_digit(10) && second.is_digit(10) && first == second
        })
        .fold(0, |acc, (x, _)| {        // Sum it all up
            acc + x.to_digit(10).unwrap()
        })
}

fn main() {
    match get_input() {
        Err(exit_code) => std::process::exit(exit_code),
        Ok(input) => {
            println!("{}", calculate_sum(&input));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::rand::{thread_rng, Rng};
    use test::Bencher;

    #[test]
    fn nonwrapping() {
        assert_eq!(calculate_sum(&String::from("122311455")), 8);
    }

    #[test]
    fn wrapping() {
        assert_eq!(calculate_sum(&String::from("1223441")), 7);
    }

    #[test]
    fn empty() {
        assert_eq!(calculate_sum(&String::from("")), 0);
    }

    #[test]
    fn non_numeric() {
        assert_eq!(calculate_sum(&String::from("3ab$11F3")), 4);
    }

    /// Ghetto generator of random digit chars. There's probably a better
    /// way to do this...
    struct RandDigits {}
    impl Iterator for RandDigits {
        type Item = char;
        fn next(&mut self) -> Option<char> {
            std::char::from_digit(thread_rng().gen_range(0, 10), 10)
        }
    }

    #[bench]
    fn bench_hundred(b: &mut Bencher) {
        let s: String = RandDigits {}.take(100).collect();
        b.iter(|| calculate_sum(&s));
    }

    #[bench]
    fn bench_ten_thousand(b: &mut Bencher) {
        let s: String = RandDigits {}.take(10000).collect();
        b.iter(|| calculate_sum(&s));
    }

    #[bench]
    fn bench_million(b: &mut Bencher) {
        let s: String = RandDigits {}.take(1000000).collect();
        b.iter(|| calculate_sum(&s));
    }
}
