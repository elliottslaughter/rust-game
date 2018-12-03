use std::cmp::{max, min};
use std::ops::{Add, Sub};

use crate::point::Point;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rect {
    pub lo: Point,
    pub hi: Point, // exclusive
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
        (self.hi.x - self.lo.x) as u32
    }

    pub fn height(self) -> u32 {
        (self.hi.y - self.lo.y) as u32
    }

    pub fn size(self) -> Point {
        (self.width(), self.height()).into()
    }

    pub fn grow(self, x: i32) -> Rect {
        Rect::new(self.lo, self.hi + x)
    }

    // Index points on the rect in increments of half the rect
    // width. I.e. 0,0 is the center of the rect, -1,-1 is the
    // top-left corner, 1,0 is the center of the right edge, etc.
    pub fn index(self, ix: i32, iy: i32) -> Point {
        self.lo + Point::new(
            ((ix + 1) * (self.hi.x - self.lo.x)) / 2,
            ((iy + 1) * (self.hi.y - self.lo.y)) / 2,
        )
    }

    pub fn center(self) -> Point {
        self.index(0, 0)
    }

    pub fn clamp(self, p: Point) -> Point {
        Point::new(
            clamp(p.x, self.lo.x, self.hi.x),
            clamp(p.y, self.lo.y, self.hi.y),
        )
    }

    pub fn rotate(self, origin: Point, angle: i32 /* degrees CCW */) -> Rect {
        // Find the four corners of the rectangle.

        // Important: account for exclusiveness of self.hi, otherwise
        // coordinates will be off if one of these gets swapped into a
        // low position. Shift back afterwards.
        let interior = self.grow(-1);
        let upper_left = interior.lo;
        let upper_right = (interior.hi.x, interior.lo.y).into();
        let lower_left = (interior.lo.x, interior.hi.y).into();
        let lower_right = interior.hi;

        // Figure out which points to rotate based on angle.
        let (lo, hi) = match angle {
            0 => (upper_left, lower_right),
            90 => (upper_right, lower_left),
            180 => (lower_right, upper_left),
            270 => (lower_left, upper_right),
            _ => unimplemented!(),
        };

        Rect::new(lo.rotate(origin, angle), hi.rotate(origin, angle)).grow(1)
    }

    pub fn has_intersection(self, r: Rect) -> bool {
        !(self.hi.x <= r.lo.x || self.hi.y <= r.lo.y || r.hi.x <= self.lo.x || r.hi.y <= self.lo.y)
    }
}

impl Default for Rect {
    fn default() -> Rect {
        ((0, 0), (0, 0)).into()
    }
}

impl From<sdl2::rect::Rect> for Rect {
    fn from(r: sdl2::rect::Rect) -> Rect {
        let lo = (r.x(), r.y()).into();
        let size: Point = (r.width(), r.height()).into();
        Rect::new(lo, lo + size)
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

impl From<(i32, i32, i32, i32)> for Rect {
    fn from(r: (i32, i32, i32, i32)) -> Rect {
        Rect::new_with_size(r.0, r.1, r.2, r.3)
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

impl Add<Point> for Rect {
    type Output = Rect;

    fn add(self, other: Point) -> Rect {
        Rect::new(self.lo + other, self.hi + other)
    }
}

impl Sub<Point> for Rect {
    type Output = Rect;

    fn sub(self, other: Point) -> Rect {
        Rect::new(self.lo - other, self.hi - other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_index() {
        let r1 = Rect::new(Point::new(-1, -2), Point::new(3, 4));
        assert_eq!(r1.index(-1, -1), r1.lo);
        assert_eq!(r1.index(1, 1), r1.hi);
        assert_eq!(r1.index(0, 0), r1.center());
        assert_eq!(r1.index(0, 1).x, r1.center().x);
        assert_eq!(r1.index(0, 1).y, r1.hi.y);
        assert_eq!(r1.index(0, -1).x, r1.center().x);
        assert_eq!(r1.index(0, -1).y, r1.lo.y);
        assert_eq!(r1.index(1, 0).x, r1.hi.x);
        assert_eq!(r1.index(1, 0).y, r1.center().y);
        assert_eq!(r1.index(-1, 0).x, r1.lo.x);
        assert_eq!(r1.index(-1, 0).y, r1.center().y);
    }

    #[test]
    fn rect_center() {
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(2, 2)).center(),
            Point::new(0, 0)
        );
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(3, 3)).center(),
            Point::new(1, 1)
        );
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(4, 4)).center(),
            Point::new(1, 1)
        );
        assert_eq!(
            Rect::new(Point::new(-1, -1), Point::new(5, 3)).center(),
            Point::new(2, 1)
        );
    }

    #[test]
    fn rect_rotate() {
        let r1 = Rect::new(Point::new(-1, -1), Point::new(2, 2));
        let o1 = Point::new(0, 0);
        assert_eq!(r1.rotate(o1, 0), r1);
        assert_eq!(r1.rotate(o1, 90), r1);
        assert_eq!(r1.rotate(o1, 180), r1);
        assert_eq!(r1.rotate(o1, 270), r1);

        let r2 = Rect::new(Point::new(-1, -1), Point::new(1, 1));
        assert_eq!(r2.rotate(o1, 0), r2);
        assert_eq!(
            r2.rotate(o1, 90),
            Rect::new(Point::new(-1, 0), Point::new(1, 2))
        );
        assert_eq!(
            r2.rotate(o1, 180),
            Rect::new(Point::new(0, 0), Point::new(2, 2))
        );
        assert_eq!(
            r2.rotate(o1, 270),
            Rect::new(Point::new(0, -1), Point::new(2, 1))
        );

        let r3 = Rect::new(Point::new(-1, -1), Point::new(3, 4));
        assert_eq!(r3.rotate(o1, 0), r3);
        assert_eq!(
            r3.rotate(o1, 90),
            Rect::new(Point::new(-1, -2), Point::new(4, 2))
        );
    }
}
