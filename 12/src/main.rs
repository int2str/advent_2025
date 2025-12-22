#![allow(dead_code)]

use itertools::Itertools;
use std::collections::HashSet;
use std::io::Result;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Shape([u8; 3]);

impl std::ops::Index<usize> for Shape {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Shape {
    fn occupied_area(&self) -> u32 {
        self.0.iter().map(|byte| byte.count_ones()).sum()
    }

    fn rotated_clockwise(&self) -> Shape {
        let mut new_bits = [0u8; 3];

        // For a 3x3 grid, rotating 90° clockwise:
        // new[row][col] = old[2-col][row]
        for (row, col) in (0..3).cartesian_product(0..3) {
            let bit = (self.0[row] >> col) & 1;
            if bit == 1 {
                let new_row = col;
                let new_col = 2 - row;
                new_bits[new_row] |= 1 << new_col;
            }
        }

        Shape(new_bits)
    }

    fn flipped_vertical(&self) -> Shape {
        Shape([self.0[2], self.0[1], self.0[0]])
    }

    fn flipped_horizontal(&self) -> Shape {
        let mut new_bits = [0u8; 3];

        for (row, col) in (0..3).cartesian_product(0..3) {
            let bit = (self.0[row] >> col) & 1;
            if bit == 1 {
                let new_col = 2 - col;
                new_bits[row] |= 1 << new_col;
            }
        }

        Shape(new_bits)
    }

    fn all_orientations(&self) -> Vec<Shape> {
        let mut orientations = HashSet::with_capacity(8);

        let mut current = *self;
        for _ in 0..4 {
            orientations.insert(current);
            orientations.insert(current.flipped_vertical());
            current = current.rotated_clockwise();
        }

        orientations.into_iter().collect()
    }
}

#[derive(Debug)]
struct Area {
    width: usize,
    height: usize,
    presents: Vec<u32>,
}

impl Area {
    fn size(&self) -> u32 {
        (self.width * self.height) as u32
    }

    fn most_presents_idx(&self) -> usize {
        self.presents
            .iter()
            .enumerate()
            .max_by_key(|(_, count)| *count)
            .unwrap()
            .0
    }

    fn all_presents_placed(&self) -> bool {
        self.presents.iter().all(|&n| n == 0)
    }
}

struct PlacementState {
    rows: Vec<u64>,
    width: usize,
    height: usize,
}

impl PlacementState {
    fn new(width: usize, height: usize) -> Self {
        PlacementState {
            rows: vec![0u64; height],
            width,
            height,
        }
    }

    fn area_total(&self) -> u32 {
        (self.width * self.height) as u32
    }

    fn area_occupied(&self) -> u32 {
        self.rows.iter().map(|row| row.count_ones()).sum()
    }

    fn can_place(&self, shape: &Shape, x: usize, y: usize) -> bool {
        // Check if shape fits within bounds
        if y + 3 > self.height || x + 3 > self.width {
            return false;
        }

        // Check each row of the shape for collisions
        self.rows
            .iter()
            .skip(y)
            .take(3)
            .enumerate()
            .all(|(row, state_row)| {
                let pattern = (shape[row] as u64) << x;
                (state_row & pattern) == 0
            })
    }

    fn place(&mut self, shape: &Shape, x: usize, y: usize) {
        self.rows
            .iter_mut()
            .skip(y)
            .take(3)
            .enumerate()
            .for_each(|(row, state_row)| {
                *state_row |= (shape[row] as u64) << x;
            });
    }

    fn remove(&mut self, shape: &Shape, x: usize, y: usize) {
        self.rows
            .iter_mut()
            .skip(y)
            .take(3)
            .enumerate()
            .for_each(|(row, state_row)| {
                *state_row &= !((shape[row] as u64) << x);
            });
    }
}

struct TreeFarm {
    shapes: Vec<Vec<Shape>>,
    trees: Vec<Area>,
}

fn read_input<P: AsRef<Path>>(path: P) -> Result<TreeFarm> {
    let content = std::fs::read_to_string(path)?;

    let mut original_shapes = Vec::new();
    let mut trees = Vec::new();

    let mut lines = content.lines().filter(|line| !line.is_empty());
    while let Some(line) = lines.next() {
        if line.contains('x') && line.contains(':') {
            // Parse Area
            let parts: Vec<&str> = line.split(':').collect();
            let dimensions: Vec<usize> = parts[0].split('x').map(|s| s.parse().unwrap()).collect();
            let presents = parts[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            trees.push(Area {
                width: dimensions[0],
                height: dimensions[1],
                presents,
            });
        } else if line.ends_with(':') {
            // Parse Shape; read next 3 lines
            let bits = [
                parse_shape_line(lines.next().unwrap()),
                parse_shape_line(lines.next().unwrap()),
                parse_shape_line(lines.next().unwrap()),
            ];
            original_shapes.push(Shape(bits));
        }
    }

    Ok(TreeFarm {
        // Pre-compute all orientations for each shape
        shapes: original_shapes
            .iter()
            .map(|s| s.all_orientations())
            .collect(),
        trees,
    })
}

fn parse_shape_line(line: &str) -> u8 {
    line.chars()
        .take(8)
        .enumerate()
        .fold(0u8, |acc, (pos, ch)| {
            if ch == '#' {
                acc | (1 << (2 - pos))
            } else {
                acc
            }
        })
}

fn can_fit_shapes(state: &mut PlacementState, farm: &mut TreeFarm, area_idx: usize) -> bool {
    if farm.trees[area_idx].all_presents_placed() {
        return true;
    }

    // Check if enough empty space is available
    let space_needed: u32 = farm.trees[area_idx]
        .presents
        .iter()
        .enumerate()
        .map(|(idx, &count)| farm.shapes[idx][0].occupied_area() * count)
        .sum();
    if state.area_occupied() + space_needed > state.area_total() {
        return false;
    }

    // Find the shape with the most remaining instances (most constrained)
    let shape_idx = farm.trees[area_idx].most_presents_idx();

    for shape in &farm.shapes[shape_idx].clone() {
        for (y, x) in (0..state.height - 2).cartesian_product(0..state.width - 2) {
            if state.can_place(shape, x, y) {
                state.place(shape, x, y);
                farm.trees[area_idx].presents[shape_idx] -= 1;

                if can_fit_shapes(state, farm, area_idx) {
                    return true;
                }

                state.remove(shape, x, y);
                farm.trees[area_idx].presents[shape_idx] += 1;
            }
        }
    }

    false
}

fn can_fit_area(farm: &mut TreeFarm, area_idx: usize) -> bool {
    let area = &farm.trees[area_idx];
    let mut state = PlacementState::new(area.width, area.height);
    can_fit_shapes(&mut state, farm, area_idx)
}

fn count_valid_areas(mut farm: TreeFarm) -> usize {
    (0..farm.trees.len())
        .filter(|&idx| can_fit_area(&mut farm, idx))
        .count()
}

fn main() {
    println!("Run 'cargo test' instead ;)");
}

#[cfg(test)]
mod tests {
    mod day12_christmas_tree_farm {
        use super::super::*;

        #[test]
        fn sample() {
            let farm = read_input("data/sample.txt").unwrap();
            assert_eq!(count_valid_areas(farm), 2);
        }

        #[test]
        #[cfg(feature = "private")]
        fn input() {
            let farm = read_input("data/input.txt").unwrap();
            assert_eq!(count_valid_areas(farm), 457);
        }
    }
}
