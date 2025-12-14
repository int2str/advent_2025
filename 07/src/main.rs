#![allow(dead_code)]

use std::io::Result;
use std::path::Path;
use utils::Grid;

fn tachy_splits<P: AsRef<Path>>(path: P) -> Result<u64> {
    let mut grid = Grid::<char>::from_file(path)?;
    let start = grid.find('S').unwrap();
    let mut rays = vec![start];
    let mut splits = 0u64;

    while let Some((x, mut y)) = rays.pop() {
        while y < grid.height() - 1 {
            y += 1;

            if grid.get(x, y).unwrap() == '^' {
                // Collect valid unvisited neighbors first
                let neighbors: Vec<_> = [x.wrapping_sub(1), x + 1]
                    .iter()
                    .filter(|&&nx| nx < grid.width())
                    .filter(|&&nx| grid.get(nx, y).unwrap() == '.')
                    .copied()
                    .collect();

                // Mark and add to queue
                neighbors.iter().for_each(|&nx| {
                    grid.set(nx, y, '|');
                    rays.push((nx, y));
                });

                if !neighbors.is_empty() {
                    splits += 1;
                }
                break;
            } else {
                // Moving straight down
                grid.set(x, y, '|');
            }
        }
    }

    Ok(splits)
}

fn count_leaves(grid: &Grid<char>, leaf_map: &Grid<usize>, x: usize, y: usize) -> usize {
    (y + 1..grid.height())
        .find_map(|scan_y| {
            // The .then() here converta a bool to an Option<T>
            (grid.get(x, scan_y).unwrap() == '^').then(|| {
                // The wrapping_sub allows us to check negative roll-off with s single filter below.
                [x.wrapping_sub(1), x + 1]
                    .iter()
                    .filter(|&&nx| nx < grid.width())
                    .map(|&nx| leaf_map.get(nx, scan_y).unwrap())
                    .sum()
            })
        })
        // If we reached the bottom of the grid, we're a leaf...
        .unwrap_or(1)
}

fn tachy_leaves<P: AsRef<Path>>(path: P) -> Result<usize> {
    let grid = Grid::<char>::from_file(path)?;
    let start = grid.find('S').unwrap();
    let mut leaf_map = Grid::new(grid.width(), grid.height(), 0usize);

    // Iterate the grid bottom up, to fill the map without recursion
    grid.coordinates_rev().for_each(|(x, y)| {
        leaf_map.set(x, y, count_leaves(&grid, &leaf_map, x, y));
    });

    Ok(leaf_map.get(start.0, start.1).unwrap())
}

fn main() {
    println!("Run 'cargo test' instead ;)");
}

#[cfg(test)]
mod tests {
    mod day07_laboratories {
        use super::super::*;

        #[test]
        fn sample() {
            assert_eq!(tachy_splits("data/sample.txt").unwrap(), 21);
            assert_eq!(tachy_leaves("data/sample.txt").unwrap(), 40);
        }

        #[test]
        #[cfg(feature = "private")]
        fn input() {
            assert_eq!(tachy_splits("data/input.txt").unwrap(), 1642);
            assert_eq!(tachy_leaves("data/input.txt").unwrap(), 47274292756692);
        }
    }
}
