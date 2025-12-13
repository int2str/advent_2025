#![allow(dead_code)]

use std::cmp::max;
use std::io::Result;
use std::path::Path;

fn combine_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    // Put all the 'from' parts in order first
    ranges.sort();

    // Now we're iterating backwards and compare to see if any two ranges
    // overlap. We then temporarily set them to (0, 0) to be filtered out
    // later.
    for i in (1..ranges.len()).rev() {
        // The -1 when comparing the 'from' part of the later range is there
        // so that we combine ranges that are directly adjacent.
        if ranges[i - 1].1 >= (ranges[i].0 - 1) {
            ranges[i - 1].1 = max(ranges[i].1, ranges[i - 1].1);
            ranges[i] = (0, 0);
        }
    }

    // Now remove the combined ranges ...
    ranges.retain_mut(|(from, to)| *from != 0 && *to != 0);
    ranges
}

fn check_ranges<P: AsRef<Path>>(path: P) -> Result<(usize, usize)> {
    let file = std::fs::read_to_string(path)?;

    let ranges = combine_ranges(
        file.lines()
            .take_while(|line| !line.is_empty())
            .map(|range| {
                let (from, to) = range.split_once('-').unwrap();
                (from.parse::<usize>().unwrap(), to.parse::<usize>().unwrap())
            })
            .collect::<Vec<(usize, usize)>>(),
    );

    let fresh = file
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .filter(|v| ranges.iter().any(|(from, to)| from <= v && v <= to))
        .count();

    let possible: usize = ranges.iter().map(|(from, to)| to - from + 1).sum();

    Ok((fresh, possible))
}

fn main() {
    println!("Run 'cargo test' instead ;)");
}

#[cfg(test)]
mod tests {
    mod day05_cafeteria {
        use super::super::*;

        #[test]
        fn sample() {
            let (fresh, possible) = check_ranges("data/sample.txt").unwrap();
            assert_eq!(fresh, 3);
            assert_eq!(possible, 14);
        }

        #[test]
        #[cfg(feature = "private")]
        fn input() {
            let (fresh, possible) = check_ranges("data/input.txt").unwrap();
            assert_eq!(fresh, 739);
            assert_eq!(possible, 344486348901788);
        }
    }
}
