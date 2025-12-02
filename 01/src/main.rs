#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn file_lines<P: AsRef<Path>>(filename: P) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn decode<P: AsRef<Path>>(path: P) -> Result<(i32, i32)> {
    let (zeros, turns, _) = file_lines(path)?
        .map(|line| {
            let line = line.unwrap();
            let clicks =
                line[1..].parse::<i32>().unwrap() * if line.starts_with('L') { -1 } else { 1 };
            ((clicks / 100).abs(), clicks - (clicks / 100) * 100)
        })
        .fold(
            (0, 0, 50),
            |(total_zeros, total_turns, old_position), (turns, clicks)| {
                let position = old_position + clicks;

                let (new_position, extra_turn) = if position < 0 {
                    (position + 100, if old_position != 0 { 1 } else { 0 })
                } else if position > 99 {
                    (position - 100, if position != 100 { 1 } else { 0 })
                } else {
                    (position, 0)
                };

                (
                    total_zeros + if new_position == 0 { 1 } else { 0 },
                    total_turns + turns + extra_turn,
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
