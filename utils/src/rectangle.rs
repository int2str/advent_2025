use crate::{Line, Point};

/// 2-dimensional axis-aligned rectangle
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Rectangle {
            top_left,
            bottom_right,
        }
    }

    pub fn from_points(p1: &Point, p2: &Point) -> Self {
        Rectangle {
            top_left: Point::new(p1.x.min(p2.x), p1.y.min(p2.y)),
            bottom_right: Point::new(p1.x.max(p2.x), p1.y.max(p2.y)),
        }
    }

    pub fn corners(&self) -> [Point; 4] {
        [
            self.top_left,
            Point::new(self.bottom_right.x, self.top_left.y),
            self.bottom_right,
            Point::new(self.top_left.x, self.bottom_right.y),
        ]
    }

    pub fn horizontal_edges(&self) -> [Line; 2] {
        [
            Line::new(
                self.top_left,
                Point::new(self.bottom_right.x, self.top_left.y),
            ),
            Line::new(
                Point::new(self.top_left.x, self.bottom_right.y),
                self.bottom_right,
            ),
        ]
    }

    pub fn vertical_edges(&self) -> [Line; 2] {
        [
            Line::new(
                self.top_left,
                Point::new(self.top_left.x, self.bottom_right.y),
            ),
            Line::new(
                Point::new(self.bottom_right.x, self.top_left.y),
                self.bottom_right,
            ),
        ]
    }

    pub fn area(&self) -> usize {
        (self.bottom_right.x - self.top_left.x + 1) * (self.bottom_right.y - self.top_left.y + 1)
    }

    pub fn is_degenerate(&self) -> bool {
        self.top_left.x == self.bottom_right.x || self.top_left.y == self.bottom_right.y
    }
}
