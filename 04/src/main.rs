#![allow(dead_code)]

use utils::Grid;

fn reachable_from(grid: &Grid<char>, x: usize, y: usize) -> usize {
    grid.neighbors(x, y)
        .into_iter()
        .filter_map(|(x, y)| grid.get(x, y))
        .map(|chr| if chr == '@' { 1usize } else { 0usize })
        .sum()
}

fn reachable_rolls(grid: &Grid<char>) -> Vec<(usize, usize)> {
    grid.coordinates()
        .filter(|(x, y)| grid.get(*x, *y) == Some('@'))
        .filter(|(x, y)| reachable_from(grid, *x, *y) < 4)
        .collect()
}

fn reachable(grid: &Grid<char>) -> usize {
    reachable_rolls(grid).len()
}

// PERF(AE): Only the neighbors of any removed roll theoretically need to be
// re-checked; not the whole grid.
fn reachable_after_remove(grid: &mut Grid<char>) -> usize {
    let mut total: usize = 0;
    loop {
        let removable = reachable_rolls(grid);
        if removable.is_empty() {
            break;
        }
        total += removable.len();
        for (x, y) in removable {
            grid.set(x, y, '.');
        }
    }
    total
}

fn main() {
    println!("Run 'cargo test' instead ;)");
}

#[cfg(test)]
mod tests {
    mod day04_printing_department {
        use super::super::*;

        #[test]
        fn sample() {
            let mut grid = Grid::<char>::from_file("data/sample.txt").unwrap();
            assert_eq!(reachable(&grid), 13);
            assert_eq!(reachable_after_remove(&mut grid), 43);
        }

        #[test]
        #[cfg(feature = "private")]
        fn input() {
            let mut grid = Grid::<char>::from_file("data/input.txt").unwrap();
            assert_eq!(reachable(&grid), 1346);
            assert_eq!(reachable_after_remove(&mut grid), 8493);
        }
    }
}
