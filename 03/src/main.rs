#![allow(dead_code)]

use std::io::Result;
use std::path::Path;

fn to_u64(chr: char) -> u64 {
    chr.to_digit(10).unwrap().into()
}

fn largest_battery(bank: &str) -> (usize, u64) {
    bank.chars()
        .rev()
        .enumerate()
        .max_by_key(|(_, chr)| *chr)
        .map(|(idx, chr)| (bank.len() - idx, to_u64(chr)))
        .unwrap()
}

fn joltage(bank: &str, take: usize) -> u64 {
    (1..=take)
        .rev()
        .fold((0usize, 0u64), |(left, sum), n| {
            let (offset, battery) = largest_battery(&bank[left..bank.len() - n + 1]);
            (left + offset, sum * 10 + battery)
        })
        .1
}

fn bms<P: AsRef<Path>>(filename: P, take: usize) -> Result<u64> {
    Ok(std::fs::read_to_string(filename)?
        .lines()
        .map(|line| joltage(line, take))
        .sum())
}

fn main() {
    println!("Run 'cargo test' instead ;)");
}

#[cfg(test)]
mod tests {
    mod day03_lobby {
        use super::super::*;

        #[test]
        fn sample() {
            assert_eq!(bms("data/sample.txt", 2).unwrap(), 357);
            assert_eq!(bms("data/sample.txt", 12).unwrap(), 3121910778619);
        }

        #[test]
        #[cfg(feature = "private")]
        fn input() {
            assert_eq!(bms("data/input.txt", 2).unwrap(), 16973);
            assert_eq!(bms("data/input.txt", 12).unwrap(), 168027167146027);
        }
    }
}
