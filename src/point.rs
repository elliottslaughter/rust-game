use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn rotate(self, origin: Point, angle: i32 /* degrees CCW */) -> Point {
        let delta = self - origin;
        let rotated = match angle {
            0 => delta,
            90 => Point::new(delta.y, -delta.x),
            180 => Point::new(-delta.x, -delta.y),
            270 => Point::new(-delta.y, delta.x),
            _ => unimplemented!(),
        };
        rotated + origin
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

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, other: i32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<i32> for Point {
    type Output = Point;

    fn div(self, other: i32) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_add() {
        assert_eq!(Point::new(1, 2) + Point::new(0, 0), Point::new(1, 2));
        assert_eq!(Point::new(1, 0) + Point::new(0, 2), Point::new(1, 2));
        assert_eq!(Point::new(0, 1) + Point::new(1, 1), Point::new(1, 2));
        assert_eq!(Point::new(1, -1) + Point::new(0, 3), Point::new(1, 2));

        assert_eq!(Point::new(0, 1) + 1, Point::new(1, 2));
    }

    #[test]
    fn point_sub() {
        assert_eq!(Point::new(1, 2) - Point::new(0, 0), Point::new(1, 2));
        assert_eq!(Point::new(1, 0) - Point::new(0, -2), Point::new(1, 2));
        assert_eq!(Point::new(0, 1) - Point::new(-1, -1), Point::new(1, 2));
        assert_eq!(Point::new(1, -1) - Point::new(0, -3), Point::new(1, 2));

        assert_eq!(Point::new(2, 3) - 1, Point::new(1, 2));
    }

    #[test]
    fn point_mul() {
        assert_eq!(Point::new(2, 3) * 2, Point::new(4, 6));
    }

    #[test]
    fn point_div() {
        assert_eq!(Point::new(4, 6) / 2, Point::new(2, 3));
    }

    #[test]
    fn point_rotate() {
        assert_eq!(
            Point::new(1, 0).rotate(Point::new(0, 0), 0),
            Point::new(1, 0)
        );
        assert_eq!(
            Point::new(0, 1).rotate(Point::new(0, 0), 0),
            Point::new(0, 1)
        );
        assert_eq!(
            Point::new(-1, 0).rotate(Point::new(0, 0), 0),
            Point::new(-1, 0)
        );
        assert_eq!(
            Point::new(0, -1).rotate(Point::new(0, 0), 0),
            Point::new(0, -1)
        );

        assert_eq!(
            Point::new(1, 0).rotate(Point::new(0, 0), 90),
            Point::new(0, -1)
        );
        assert_eq!(
            Point::new(0, 1).rotate(Point::new(0, 0), 90),
            Point::new(1, 0)
        );
        assert_eq!(
            Point::new(-1, 0).rotate(Point::new(0, 0), 90),
            Point::new(0, 1)
        );
        assert_eq!(
            Point::new(0, -1).rotate(Point::new(0, 0), 90),
            Point::new(-1, 0)
        );

        assert_eq!(
            Point::new(1, 0).rotate(Point::new(0, 0), 180),
            Point::new(-1, 0)
        );
        assert_eq!(
            Point::new(0, 1).rotate(Point::new(0, 0), 180),
            Point::new(0, -1)
        );
        assert_eq!(
            Point::new(-1, 0).rotate(Point::new(0, 0), 180),
            Point::new(1, 0)
        );
        assert_eq!(
            Point::new(0, -1).rotate(Point::new(0, 0), 180),
            Point::new(0, 1)
        );

        assert_eq!(
            Point::new(1, 0).rotate(Point::new(0, 0), 270),
            Point::new(0, 1)
        );
        assert_eq!(
            Point::new(0, 1).rotate(Point::new(0, 0), 270),
            Point::new(-1, 0)
        );
        assert_eq!(
            Point::new(-1, 0).rotate(Point::new(0, 0), 270),
            Point::new(0, -1)
        );
        assert_eq!(
            Point::new(0, -1).rotate(Point::new(0, 0), 270),
            Point::new(1, 0)
        );

        assert_eq!(
            Point::new(1, 2).rotate(Point::new(0, 0), 90),
            Point::new(2, -1)
        );
        assert_eq!(
            Point::new(-1, 2).rotate(Point::new(0, 0), 90),
            Point::new(2, 1)
        );
        assert_eq!(
            Point::new(-1, -2).rotate(Point::new(0, 0), 90),
            Point::new(-2, 1)
        );
        assert_eq!(
            Point::new(1, -2).rotate(Point::new(0, 0), 90),
            Point::new(-2, -1)
        );

        assert_eq!(
            Point::new(2, 3).rotate(Point::new(1, 1), 90),
            Point::new(3, 0)
        );
        assert_eq!(
            Point::new(-2, 3).rotate(Point::new(1, 1), 90),
            Point::new(3, 4)
        );
    }
}
