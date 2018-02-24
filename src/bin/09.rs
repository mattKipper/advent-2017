use std::str::Chars;

// This problem can be modelled as an FSM, with transitions defined
// by the next character in the stream.
//
// In the FSM below, the labeled transitions (e.g. ---'!'--->) are
// taken when the label character is encountered. If no labeled
// character appers, the unlabeled transition (i.e. no '') is taken
//         ___                 ___
//         | V                 | V
//      ----------        -------------
//      | Normal |--'<'-->|  Garbage  |<-----------|
//      ----------        -------------            |
//             ^             |   |             ----------
//             |_____'>'_____|   |-----'!'---->|  Null  |
//                                             ----------
// The unlabeled transition from the "Normal" state also has side 
// effects 
//   1. '{' signifies a new group, and since the problem guarantees
//       that all inputs are well-formed (i.e. all groups are closed), the 
//       total score is immediately increased by the current depth, and the
//       depth is incremented. Likewise
//   2. '}' signifies the end of a group. Since we don't need to check that
//      groups are successfully closed, the side effect is to simply 
//      decrement the depth. 
fn score(mut stream: Chars) -> u32 {

    let mut total_score = 0;
    let mut next_group_score = 1;
    let mut in_garbage = false;

    while let Some(ch) = stream.next() {

        if !in_garbage {
            match &ch {
                &'{' => {
                    total_score += next_group_score;
                    next_group_score += 1;
                },
                &'}' => next_group_score -= 1,
                &'<' => in_garbage = true,
                &_ => {},
            };
        }
        else {
            match &ch {
                &'!' => {stream.next();},
                &'>' => in_garbage = false,
                &_ => {},
            };
        };
    };

    total_score
}

fn print_usage() {
    println!("Day 9: Stream Processing");
    println!("Usage:");
    println!("09 [-h / --help] <input_string>");
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

fn main() {
    match get_input() {
        Ok(input) => println!("{}", score(input.chars())),
        Err(exit_code) => std::process::exit(exit_code),
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_examples() {
        let examples = [
            ("{}", 1),
            ("{{{}}}", 6),
            ("{{},{}}", 5),
            ("{{{},{},{{}}}}", 16),
            ("{<a>,<a>,<a>,<a>}", 1),
            ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
            ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
            ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3)
        ];

        for &(stream, correct_score) in examples.iter() {
            assert_eq!(score(stream.chars()), correct_score);
        }
    }
}