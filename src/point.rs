use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl From<(i32, i32)> for Point {
    fn from(p: (i32, i32)) -> Point {
        Point { x: p.0, y: p.1 }
    }
}

impl From<(u32, u32)> for Point {
    fn from(p: (u32, u32)) -> Point {
        Point {
            x: p.0 as i32,
            y: p.1 as i32,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<i32> for Point {
    type Output = Point;

    fn add(self, other: i32) -> Point {
        self + Point::new(other, other)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<i32> for Point {
    type Output = Point;

    fn sub(self, other: i32) -> Point {
        self - Point::new(other, other)
    }
}
