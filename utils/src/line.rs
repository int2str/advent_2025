use crate::Point;

/// 2-dimensional line
#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct Line {
    pub from: Point,
    pub to: Point,
}

impl Line {
    pub fn new(from: Point, to: Point) -> Self {
        Line { from, to }
    }

    pub fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    pub fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    pub fn contains_x(&self, x: usize) -> bool {
        let (min_x, max_x) = self.x_range();
        min_x <= x && x <= max_x
    }

    pub fn contains_y(&self, y: usize) -> bool {
        let (min_y, max_y) = self.y_range();
        min_y <= y && y <= max_y
    }

    pub fn is_endpoint(&self, point: &Point) -> bool {
        point == &self.from || point == &self.to
    }

    fn x_range(&self) -> (usize, usize) {
        (self.from.x.min(self.to.x), self.from.x.max(self.to.x))
    }

    fn y_range(&self) -> (usize, usize) {
        (self.from.y.min(self.to.y), self.from.y.max(self.to.y))
    }
}
