use std::env::args;

fn spiral_coords(value: u32) -> (i64, i64) {
    // For loop n of the spiral, the highest number (N) is given by
    // (2n + 1)^2
    let n = (((value as f64).sqrt() - 1.0) / 2.0).ceil() as i64;
    let max = (2 * n + 1).pow(2) as i64;

    // All numers in loop n fall on a square between
    // (n,n), (-n,n), (n,-n) and (-n,-n). We can determine
    // which line of the rectangle the value falls on in terms
    // of the loop's maximum
    //
    // (n,-n) -> (-n,-n) = max - 2n <= value <=max
    let value = value as i64;
    if value >= max - 2 * n {
        (n - (max - value), -n)
    }
    // (-n,-n) -> (-n,n) = max - 4n <= value <= max - 2n
    else if value >= max - 4 * n {
        (-n, -n + (max - value - 2 * n))
    }
    // (-n,n) -> (n, n) = max - 6n <= value <= max - 4n
    else if value >= max - 6 * n {
        (-n + (max - value - 4 * n), n)
    }
    // (n,n) -> (n,-n) = max - 8n <= value <= max - 6n (default)
    else {
        (n, n - (max - value - 6 * n))
    }
}

/// Taxicab distance between two cartesian points
fn distance((x0, y0): (i64, i64), (x1, y1): (i64, i64)) -> i64 {
    (x0 - x1).abs() + (y0 - y1).abs()
}

fn print_usage() {
    println!("Day 3: Spiral Memory");
    println!("Usage:");
    println!("03 <num>");
    println!("  <num> - Unsigned integer");
}

fn main() {
    match args().nth(1) {
        Some(input) => match input.parse::<u32>() {
            Ok(input) => {
                println!("{}", distance(spiral_coords(input), (0, 0)));
            }
            Err(_) => {
                println!("Input must be an unsigned integer!");
                std::process::exit(-1);
            }
        },

        None => {
            print_usage();
            std::process::exit(-1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPoint {
        x: i64,
        y: i64,
        distance: i64,
    }

    impl TestPoint {
        pub fn coords(&self) -> (i64, i64) {
            (self.x, self.y)
        }

        pub fn distance(&self) -> i64 {
            self.distance
        }
    }

    // A diagram of 1-23 is shown on the AoC page. Test that the
    // coordinates and distance to origin for all of these
    // can be calculated
    #[test]
    fn one_to_twentythree() {
        let test_points = vec![
            TestPoint {
                x: 0,
                y: 0,
                distance: 0,
            }, // 1
            TestPoint {
                x: 1,
                y: 0,
                distance: 1,
            }, // 2
            TestPoint {
                x: 1,
                y: 1,
                distance: 2,
            }, // 3
            TestPoint {
                x: 0,
                y: 1,
                distance: 1,
            }, // 4
            TestPoint {
                x: -1,
                y: 1,
                distance: 2,
            }, // 5
            TestPoint {
                x: -1,
                y: 0,
                distance: 1,
            }, // 6
            TestPoint {
                x: -1,
                y: -1,
                distance: 2,
            }, // 7
            TestPoint {
                x: 0,
                y: -1,
                distance: 1,
            }, // 8
            TestPoint {
                x: 1,
                y: -1,
                distance: 2,
            }, // 9
            TestPoint {
                x: 2,
                y: -1,
                distance: 3,
            }, // 10
            TestPoint {
                x: 2,
                y: 0,
                distance: 2,
            }, // 11
            TestPoint {
                x: 2,
                y: 1,
                distance: 3,
            }, // 12
            TestPoint {
                x: 2,
                y: 2,
                distance: 4,
            }, // 13
            TestPoint {
                x: 1,
                y: 2,
                distance: 3,
            }, // 14
            TestPoint {
                x: 0,
                y: 2,
                distance: 2,
            }, // 15
            TestPoint {
                x: -1,
                y: 2,
                distance: 3,
            }, // 16
            TestPoint {
                x: -2,
                y: 2,
                distance: 4,
            }, // 17
            TestPoint {
                x: -2,
                y: 1,
                distance: 3,
            }, // 18
            TestPoint {
                x: -2,
                y: 0,
                distance: 2,
            }, // 19
            TestPoint {
                x: -2,
                y: -1,
                distance: 3,
            }, // 20
            TestPoint {
                x: -2,
                y: -2,
                distance: 4,
            }, // 21
            TestPoint {
                x: -1,
                y: -2,
                distance: 3,
            }, // 22
            TestPoint {
                x: 0,
                y: -2,
                distance: 2,
            }, // 23
        ];


        let origin = (0, 0);
        for (i, test_point) in test_points.iter().enumerate() {
            let coord = spiral_coords(i as u32 + 1); // test_points[0] corresponds to '1'
            assert_eq!(coord, test_point.coords());
            assert_eq!(distance(coord, origin), test_point.distance());
        }
    }

    // AoC example distances
    #[test]
    fn example_distances() {
        let origin = (0, 0);
        let examples = vec![(1, 0), (12, 3), (23, 2), (1024, 31)];
        for (num, dist) in examples {
            assert_eq!(distance(spiral_coords(num), origin), dist);
        }
    }
}
