use std::io::{Error, ErrorKind, Result};
use std::path::Path;

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> Self
    where
        T: Clone,
    {
        Grid {
            data: vec![default_value; width * height],
            width,
            height,
        }
    }

    pub fn from_file<P>(path: P) -> Result<Grid<char>>
    where
        P: AsRef<Path>,
    {
        let mut data: Vec<char> = Vec::new();
        let mut width: usize = 0;
        let mut height: usize = 0;

        for line in std::fs::read_to_string(path)?.lines() {
            if width == 0 {
                width = line.len();
            }
            if line.len() != width {
                return Err(Error::new(ErrorKind::InvalidData, ""));
            }
            line.chars().for_each(|chr| data.push(chr));
            height += 1;
        }

        Ok(Grid {
            data,
            width,
            height,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T>
    where
        T: Clone,
    {
        if x < self.width && y < self.height {
            Some(self.data[y * self.width + x].clone())
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> bool {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
            true
        } else {
            false
        }
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let directions: [(usize, usize); 8] = [
            (x.wrapping_sub(1), y.wrapping_sub(1)),
            (x, y.wrapping_sub(1)),
            (x.wrapping_add(1), y.wrapping_sub(1)),
            (x.wrapping_sub(1), y),
            (x.wrapping_add(1), y),
            (x.wrapping_sub(1), y.wrapping_add(1)),
            (x, y.wrapping_add(1)),
            (x.wrapping_add(1), y.wrapping_add(1)),
        ];
        directions
            .into_iter()
            .filter(|(x, y)| *x < self.width && *y < self.height)
            .collect()
    }

    pub fn coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| (x, y)))
    }
}

impl std::fmt::Debug for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "      ")?;
        for x in 0..self.width {
            write!(f, "{}", x % 10)?;
        }
        writeln!(f)?;

        write!(f, "      ")?;
        for _ in 0..self.width {
            write!(f, "-")?;
        }
        writeln!(f)?;

        for y in 0..self.height {
            write!(f, "{:3} | ", y)?;
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y).unwrap())?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
