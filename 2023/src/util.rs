use std::fmt::{Debug, Display};

use num_traits::{Num, cast, NumCast};

pub trait NumTraits = Num + NumCast + Default + Copy + Display;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Point<T: NumTraits> {
    pub x: T,
    pub y: T,
}

impl<T: NumTraits> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn dist_manhattan(self, other: Self) -> T {
        (self.x - other.x) + (self.y - other.y) 
    }

    pub fn dist_euclidean(self, other: Self) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let squared = x * x + y * y;

        cast::<T, f64>(squared).map(f64::sqrt).unwrap_or(0.0)
    }
}

impl<S: NumCast, D: NumTraits> From<(S, S)> for Point<D> {
    fn from((x, y): (S, S)) -> Self {
        Point {x: cast::<S, D>(x).unwrap_or_default(), y: cast::<S, D>(y).unwrap_or_default()}
    }
}

impl<T: NumTraits> Debug for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}
