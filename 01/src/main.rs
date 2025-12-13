#![allow(dead_code)]

use std::io::Result;
use std::path::Path;

fn decode<P: AsRef<Path>>(path: P) -> Result<(i32, i32)> {
    // PERFORMANCE(AE): I'm sure (***), combining map() calls would be faster here.
    // But leaving individual map() calls here for clarity. Until performance becomes
    // an issue ;).
    let (zeros, turns, _) = std::fs::read_to_string(path)?
        .lines()
        // Extract number
        .map(|line| line[1..].parse::<i32>().unwrap() * if line.starts_with('L') { -1 } else { 1 })
        // Calculate full rotations and remaining steps
        .map(|clicks| ((clicks / 100).abs(), clicks % 100))
        // Apply the steps from a given starting position
        .fold(
            (0, 0, 50),
            |(zeros, turns, old_position), (new_turns, clicks)| {
                let position = old_position + clicks;

                // This logic gets messy because it needs to prevent double counting of rotations
                // when we either started at 0 or ended there...
                let (new_position, extra_turn) = if position < 0 {
                    (position + 100, if old_position != 0 { 1 } else { 0 })
                } else if position > 99 {
                    (position - 100, if position != 100 { 1 } else { 0 })
                } else {
                    (position, 0)
                };

                (
                    zeros + if new_position == 0 { 1 } else { 0 },
                    turns + new_turns + extra_turn,
                    new_position,
                )
            },
        );
    Ok((zeros, turns + zeros))
}

fn main() {
    println!("Run 'cargo test' instead ;)");
}

#[cfg(test)]
mod tests {
    mod day01_secret_entrance {
        use super::super::*;

        #[test]
        fn sample() {
            assert_eq!(decode("data/sample.txt").unwrap(), (3, 6));
        }

        #[test]
        #[cfg(feature = "private")]
        fn input() {
            assert_eq!(decode("data/input.txt").unwrap(), (1135, 6558));
        }
    }
}
