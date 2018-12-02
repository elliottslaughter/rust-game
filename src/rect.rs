use std::cmp::{max, min};

use crate::point::Point;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rect {
    pub lo: Point,
    pub hi: Point, // inclusive
}

fn clamp<T: Ord>(x: T, v0: T, v1: T) -> T {
    min(max(x, v0), v1)
}

impl Rect {
    pub fn new(lo: Point, hi: Point) -> Rect {
        Rect { lo, hi }
    }

    pub fn new_with_size(x: i32, y: i32, width: i32, height: i32) -> Rect {
        let lo = Point::new(x, y);
        let hi = lo + Point::new(width, height) - 1;
        Rect::new(lo, hi)
    }

    pub fn width(self) -> u32 {
        (self.hi.x - self.lo.x + 1) as u32
    }

    pub fn height(self) -> u32 {
        (self.hi.y - self.lo.y + 1) as u32
    }

    pub fn size(self) -> Point {
        (self.width(), self.height()).into()
    }

    pub fn center(self) -> Point {
        (self.lo + self.hi + 1) / 2
    }

    pub fn clamp(self, p: Point) -> Point {
        Point::new(
            clamp(p.x, self.lo.x, self.hi.x),
            clamp(p.y, self.lo.y, self.hi.y),
        )
    }

    pub fn rotate(self, origin: Point, angle: i32 /* degrees CCW */) -> Rect {
        // Find the four corners of the rectangle.
        let upper_left = self.lo;
        let upper_right = (self.hi.x, self.lo.y).into();
        let lower_left = (self.lo.x, self.hi.y).into();
        let lower_right = self.hi;

        // Figure out which points to rotate based on angle.
        let (lo, hi) = match angle {
            0 => (upper_left, lower_right),
            90 => (upper_right, lower_left),
            180 => (lower_right, upper_left),
            270 => (lower_left, upper_right),
            _ => unimplemented!(),
        };

        Rect::new(lo.rotate(origin, angle), hi.rotate(origin, angle))
    }

    pub fn has_intersection(self, r: Rect) -> bool {
        !(self.hi.x < r.lo.x || self.hi.y < r.lo.y || r.hi.x < self.lo.x || r.hi.y < self.lo.y)
    }
}

impl Default for Rect {
    fn default() -> Rect {
        ((0, 0), (-1, -1)).into()
    }
}

impl From<sdl2::rect::Rect> for Rect {
    fn from(r: sdl2::rect::Rect) -> Rect {
        let lo = (r.x(), r.y()).into();
        let size: Point = (r.width(), r.height()).into();
        Rect::new(lo, lo + size - 1)
    }
}

impl From<Rect> for sdl2::rect::Rect {
    fn from(r: Rect) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(r.lo.x, r.lo.y, r.width(), r.height())
    }
}

impl From<(Point, Point)> for Rect {
    fn from(r: (Point, Point)) -> Rect {
        Rect::new(r.0, r.1)
    }
}

impl From<((i32, i32), (i32, i32))> for Rect {
    fn from(r: ((i32, i32), (i32, i32))) -> Rect {
        let lo: Point = r.0.into();
        let hi: Point = r.1.into();
        (lo, hi).into()
    }
}

impl From<((i32, i32), (u32, u32))> for Rect {
    fn from(r: ((i32, i32), (u32, u32))) -> Rect {
        let lo: Point = r.0.into();
        let hi: Point = r.1.into();
        (lo, hi).into()
    }
}

impl From<(Point, (i32, i32))> for Rect {
    fn from(r: (Point, (i32, i32))) -> Rect {
        let hi: Point = r.1.into();
        (r.0, hi).into()
    }
}

impl From<((i32, i32), Point)> for Rect {
    fn from(r: ((i32, i32), Point)) -> Rect {
        let lo: Point = r.0.into();
        (lo, r.1).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_center() {
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(1, 1)).center(),
            Point::new(0, 0)
        );
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(2, 2)).center(),
            Point::new(1, 1)
        );
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(3, 3)).center(),
            Point::new(1, 1)
        );
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(4, 2)).center(),
            Point::new(2, 1)
        );
    }

    #[test]
    fn rect_rotate() {
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(1, 1)).rotate(Point::new(0, 0), 0),
            Rect::new(Point::new(-1, -1), Point::new(1, 1))
        );
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(1, 1)).rotate(Point::new(0, 0), 90),
            Rect::new(Point::new(-1, -1), Point::new(1, 1))
        );
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(1, 1)).rotate(Point::new(0, 0), 180),
            Rect::new(Point::new(-1, -1), Point::new(1, 1))
        );
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(1, 1)).rotate(Point::new(0, 0), 270),
            Rect::new(Point::new(-1, -1), Point::new(1, 1))
        );

        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(2, 3)).rotate(Point::new(0, 0), 0),
            Rect::new(Point::new(-1, -1), Point::new(2, 3))
        );
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(2, 3)).rotate(Point::new(0, 0), 90),
            Rect::new(Point::new(-1, -2), Point::new(3, 1))
        );
    }
}
