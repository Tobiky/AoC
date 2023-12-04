#[derive(Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn dist_manhattan(self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn dist_euclidean(self, other: Self) -> f64 {
        let squared = self.x.abs_diff(other.y).pow(2) + self.y.abs_diff(other.y).pow(2);

        (squared as f64).sqrt()
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point {x, y}
    }
}
