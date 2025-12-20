#![allow(dead_code)]

use itertools::Itertools;

use std::io::Result;
use std::path::Path;
use utils::{Line, Point, Rectangle};

fn read_points<P: AsRef<Path>>(path: P) -> Result<Vec<Point>> {
    let mut points: Vec<Point> = std::fs::read_to_string(path)?
        .lines()
        .map(|line| {
            let pt: Vec<usize> = line
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect();
            assert!(pt.len() == 2);
            Point::new(pt[0], pt[1])
        })
        .collect();

    // Add the first point to the back to be able to iterate over
    // the points and still end up with a closed polygon.
    points.push(*points.first().unwrap());

    Ok(points)
}

// Assumes one line is horizontal and the other is vertical
fn lines_intersection_point(line1: &Line, line2: &Line) -> Option<Point> {
    let (horizontal, vertical) = if line1.is_horizontal() {
        (line1, line2)
    } else {
        (line2, line1)
    };

    (vertical.contains_y(horizontal.from.y) && horizontal.contains_x(vertical.from.x))
        .then_some(Point::new(vertical.from.x, horizontal.from.y))
}

// Check if intersection should be ignored (it's at a rectangle edge endpoint or a polygon vertex)
fn is_valid_intersection(rect_edge: &Line, outline_edge: &Line, points: &[Point]) -> bool {
    lines_intersection_point(rect_edge, outline_edge)
        .filter(|p| !rect_edge.is_endpoint(p))
        .is_some_and(|p| !points.contains(&p))
}

// Assumes one set of lines is horizontal and the other is vertical
fn lines_intersect(rect_edges: &[Line], outline_edges: &[Line], points: &[Point]) -> bool {
    // PERFORMANCE(AE): Tried this with itertools::cartesian_product, and it was a lot slower :'(
    for rect_edge in rect_edges {
        for outline_edge in outline_edges {
            if is_valid_intersection(rect_edge, outline_edge, points) {
                return true;
            }
        }
    }
    false
}

// Ray casting algorithm to check if a point is inside a polygon
fn point_in_polygon(point: &Point, points: &[Point]) -> bool {
    let mut inside = false;

    for (pt1, pt2) in points.iter().tuple_windows() {
        // Check if point is exactly on a vertex
        if point == pt1 || point == pt2 {
            return true;
        }

        let (x1, y1) = (pt1.x as i64, pt1.y as i64);
        let (x2, y2) = (pt2.x as i64, pt2.y as i64);
        let (py, px) = (point.y as i64, point.x as i64);

        if ((y1 > py) != (y2 > py)) && (px < (x2 - x1) * (py - y1) / (y2 - y1) + x1) {
            inside = !inside;
        }
    }
    inside
}

fn lines_from_points(points: &[Point]) -> (Vec<Line>, Vec<Line>) {
    points
        .iter()
        .tuple_windows()
        .map(|(from, to)| Line::new(*from, *to))
        .fold((Vec::new(), Vec::new()), |mut lines, line| {
            if line.is_horizontal() {
                lines.0.push(line);
            } else {
                lines.1.push(line);
            }
            lines
        })
}

fn larges_inside_rectangle(points: &[Point]) -> usize {
    let (lines_horizontal, lines_vertical) = lines_from_points(points);

    points
        .iter()
        .skip(1)
        .combinations(2)
        .map(|pair| Rectangle::from_points(pair[0], pair[1]))
        .filter(|rect| !rect.is_degenerate())
        // No line intersections with vertical or horizontal lines
        .filter(|rect| {
            !lines_intersect(&rect.horizontal_edges(), &lines_vertical, points)
                && !lines_intersect(&rect.vertical_edges(), &lines_horizontal, points)
        })
        // All four corners inside the polygon
        .filter(|rect| {
            rect.corners()
                .iter()
                .all(|corner| point_in_polygon(corner, points))
        })
        .map(|rect| rect.area())
        .max()
        .unwrap()
}

fn largest_square(points: &[Point]) -> usize {
    points
        .iter()
        .skip(1)
        .combinations(2)
        .map(|pair| Rectangle::from_points(pair[0], pair[1]))
        .map(|rect| rect.area())
        .max()
        .unwrap()
}

fn main() {
    println!("Run 'cargo test' instead ;)");
}

#[cfg(test)]
mod tests {
    mod day09_movie_theater {
        use super::super::*;

        #[test]
        fn sample() {
            let points = read_points("data/sample.txt").unwrap();
            assert_eq!(largest_square(&points), 50);
            assert_eq!(larges_inside_rectangle(&points), 24);
        }

        #[test]
        #[cfg(feature = "private")]
        fn input() {
            let points = read_points("data/input.txt").unwrap();
            assert_eq!(largest_square(&points), 4740155680);
            assert_eq!(larges_inside_rectangle(&points), 1543501936);
        }
    }
}
