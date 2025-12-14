#![allow(dead_code)]

use utils::Grid;
use utils::math::transpose;

use std::io::Result;
use std::path::Path;

fn human_math<P: AsRef<Path>>(path: P) -> Result<u64> {
    Ok(
        // Read and transpose values
        transpose(
            std::fs::read_to_string(path)?
                .lines()
                .map(|line| line.split_whitespace().collect())
                .collect(),
        )
        .into_iter()
        // Then parse as numbers and perform the desired operation
        .map(|line| {
            let iter = line
                .iter()
                .take(line.len() - 1)
                .map(|num| num.parse::<u64>().unwrap());
            match *line.last().unwrap() {
                "+" => iter.sum(),
                "*" => iter.product(),
                _ => 0u64,
            }
        })
        .sum(),
    )
}

// Takes a vertical slice of characters and parses it into a number
fn ceph_operand(grid: &Grid<char>, x: usize) -> u64 {
    (0..grid.height() - 1)
        .map(|y| grid.get(x, y).unwrap())
        .filter(|chr| chr.is_ascii_digit())
        .map(|chr| (chr as u8 - b'0') as u64)
        .fold(0u64, |acc, digit| acc * 10 + digit)
}

fn ceph_math<P: AsRef<Path>>(path: P) -> Result<u64> {
    let grid = Grid::<char>::from_file(path)?;

    Ok((0..grid.width())
        .rev()
        .fold((0u64, Vec::<u64>::new()), |(total, mut ops), x| {
            let op = ceph_operand(&grid, x);
            ops.push(op);

            (
                match grid.get(x, grid.height() - 1).unwrap() {
                    '+' => ops.iter().sum(),
                    '*' => ops.iter().product(),
                    _ => {
                        if op == 0 {
                            ops.clear();
                        }
                        0
                    }
                } + total,
                ops,
            )
        })
        .0)
}

fn main() {
    println!("Run 'cargo test' instead ;)");
}

#[cfg(test)]
mod tests {
    mod day06_trash_compactor {
        use super::super::*;

        #[test]
        fn sample() {
            assert_eq!(human_math("data/sample.txt").unwrap(), 4277556);
            assert_eq!(ceph_math("data/sample.txt").unwrap(), 3263827);
        }

        #[test]
        #[cfg(feature = "private")]
        fn input() {
            assert_eq!(human_math("data/input.txt").unwrap(), 6169101504608);
            assert_eq!(ceph_math("data/input.txt").unwrap(), 10442199710797);
        }
    }
}
