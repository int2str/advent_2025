#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;
use std::io::Result;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Debug, PartialEq)]
struct Link {
    pub from: Point,
    pub to: Point,
    pub distance: usize,
}

impl Point {
    fn distance_to(&self, other: &Point) -> usize {
        (self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2))
        .isqrt()
    }
}

fn read_points<P: AsRef<Path>>(path: P) -> Result<Vec<Point>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|line| {
            let pt: Vec<usize> = line
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect();
            assert!(pt.len() == 3);
            Point {
                x: pt[0],
                y: pt[1],
                z: pt[2],
            }
        })
        .collect())
}

fn distances_sorted(points: &[Point]) -> Vec<Link> {
    let mut distances: Vec<Link> = (0..points.len() - 1)
        .flat_map(|i| (i + 1..points.len()).map(move |j| (i, j)))
        .map(|(from, to)| Link {
            from: points[from],
            to: points[to],
            distance: points[from].distance_to(&points[to]),
        })
        .collect();
    distances.sort_by(|a, b| a.distance.cmp(&b.distance));
    distances
}

fn lowest_first(first: usize, second: usize) -> (usize, usize) {
    (std::cmp::min(first, second), std::cmp::max(first, second))
}

fn circuit_index(points: &[Point]) -> (HashMap<Point, usize>, Vec<Vec<Point>>) {
    let mut index = HashMap::new();
    let mut circuits: Vec<Vec<Point>> = Vec::new();

    for point in points.iter() {
        index.insert(*point, circuits.len());
        circuits.push(vec![*point]);
    }

    (index, circuits)
}

fn combine_circuits(
    index: &mut HashMap<Point, usize>,
    circuits: &mut [Vec<Point>],
    first: Point,
    second: Point,
) {
    // We're always combining circuits "down" to make sure the largest circuit
    // will form at index 0 ...
    let (to, from) = lowest_first(index[&first], index[&second]);
    if from != to {
        index
            .iter_mut()
            .filter(|(_, v)| **v == from)
            .for_each(|(_, v)| *v = to);

        let from_items = std::mem::take(&mut circuits[from]);
        circuits[to].extend(from_items);
    }
}

fn largest_circuits_product(circuits: &[Vec<Point>]) -> usize {
    let mut circuit_length: Vec<usize> = circuits.iter().map(|circuit| circuit.len()).collect();
    circuit_length.sort();
    circuit_length.into_iter().rev().take(3).product::<usize>()
}

fn junctions_complete(points: Vec<Point>, first_n_count: usize) -> (usize, usize) {
    let points_count = points.len();
    let (mut index, mut circuits) = circuit_index(&points);
    let mut first_n = 0;

    for (count, link) in distances_sorted(&points).into_iter().enumerate() {
        combine_circuits(&mut index, &mut circuits, link.from, link.to);

        if count == first_n_count - 1 {
            first_n = largest_circuits_product(&circuits);
        }

        if circuits[0].len() == points_count {
            return (first_n, link.from.x * link.to.x);
        }
    }

    unreachable!();
}

fn main() {
    println!("Run 'cargo test' instead ;)");
}

#[cfg(test)]
mod tests {
    mod day08_playground {
        use super::super::*;

        #[test]
        fn sample() {
            let (first_n, all) = junctions_complete(read_points("data/sample.txt").unwrap(), 10);
            assert_eq!(first_n, 40);
            assert_eq!(all, 25272);
        }

        #[test]
        #[cfg(feature = "private")]
        fn input() {
            let (first_n, all) = junctions_complete(read_points("data/input.txt").unwrap(), 1_000);
            assert_eq!(first_n, 79056);
            assert_eq!(all, 4639477);
        }
    }
}
