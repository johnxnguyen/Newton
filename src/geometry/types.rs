use std::ops::{Add, AddAssign, Div, Mul};
use std::cmp::PartialEq;
use self::Quadrant::{NW, NE, SW, SE};

// Point /////////////////////////////////////////////////////////////////////
//
// Coordinates in 2D space.

#[derive(Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn origin() -> Point {
        Point::new(0.0, 0.0)
    }

    pub fn is_origin(&self) -> bool {
        self == &Point::origin()
    }

    pub fn distance_to(&self, other: &Point) -> f32 {
        let difference = Vector::difference(self, other);
        difference.magnitude()
    }
}

// Vector ////////////////////////////////////////////////////////////////////
//
// Change of coordinates in 2D space.

#[derive(Clone, Debug)]
pub struct Vector {
    pub dx: f32,
    pub dy: f32,
}

impl PartialEq for Vector {
    fn eq(&self, other: &'_ Vector) -> bool {
        let e = 0.0000001;
        let x = (self.dx - other.dx).abs();
        let y = (self.dy - other.dy).abs();
        x < e && y < e
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
    }
}

impl<'a> Mul<f32> for &'a Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector {
            dx: self.dx * scalar,
            dy: self.dy * scalar,
        }
    }
}

impl<'a> Div<f32> for &'a Vector {
    type Output = Vector;

    fn div(self, scalar: f32) -> Self::Output {
        Vector {
            dx: self.dx / scalar,
            dy: self.dy / scalar,
        }
    }
}

impl<'a> Mul for &'a Vector {
    type Output = f32;

    fn mul(self, rhs: &Vector) -> Self::Output {
        self.dx * rhs.dx + self.dy * rhs.dy
    }
}

impl Vector {
    pub fn zero() -> Vector {
        Vector { dx: 0.0, dy: 0.0 }
    }

    pub fn difference(lhs: &Point, rhs: &Point) -> Vector {
        Vector {
            dx: (lhs.x - rhs.x),
            dy: (lhs.y - rhs.y),
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.dx * self.dx + self.dy * self.dy).sqrt()
    }

    pub fn normalized(&self) -> Option<Vector> {
        if self == &Vector::zero() {
            return None;
        }
        let magnitude = self.magnitude();

        Some(Vector {
            dx: self.dx / magnitude,
            dy: self.dy / magnitude,
        })
    }
}

// Size //////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Size {
        if width <= 0.0 || height <= 0.0 {
            panic!("A size's width and/or height must be positive. Got ({:?}, {:?})", width, height);
        }
        Size {
            width,
            height,
        }
    }
}

// Quadrant //////////////////////////////////////////////////////////////////
//
// The four quadrants of a rectangle.

#[derive(Clone, PartialEq, Debug)]
pub enum Quadrant { NW(Rect), NE(Rect), SW(Rect), SE(Rect) }

// Rect //////////////////////////////////////////////////////////////////////
//
// A rectangle whose origin denotes the position of the bottom left corner.

#[derive(Clone, PartialEq, Debug)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            origin: Point::new(x, y),
            size: Size::new(width, height),
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.origin.x && point.y >= self.origin.y &&
            point.x <= self.upper_bound().x && point.y <= self.upper_bound().y
    }

    pub fn quadrants(&self) -> (Rect, Rect, Rect, Rect) {
        let southwest = self.quarter_sized();
        let size = southwest.size.clone();

        let mut southeast = southwest.clone();
        southeast.origin.x += size.width;

        let mut northeast = southeast.clone();
        northeast.origin.y += size.height;

        let mut northwest = northeast.clone();
        northwest.origin.x -= size.width;

        (northwest, northeast, southwest, southeast)
    }

    pub fn which_quadrant(&self, point: &Point) -> Option<Quadrant> {
        // TODO: rename this subspaces
        let (nw, ne, sw, se) = self.quadrants();
        if nw.contains(point) { return Some((NW(nw))); }
        if ne.contains(point) { return Some((NE(ne))); }
        if sw.contains(point) { return Some((SW(sw))); }
        if se.contains(point) { return Some((SE(se))); }
        None
    }

    fn quarter_sized(&self) -> Rect {
        let (w, h) = (self.size.width / 2.0, self.size.height / 2.0);
        Rect::new(self.origin.x, self.origin.y, w, h)
    }

    fn upper_bound(&self) -> Point {
        Point {
            x: self.origin.x + self.size.width,
            y: self.origin.y + self.size.height,
        }
    }
}

// Tests /////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_distance_from_origin() {
        // given
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 5.0, y: 0.0 };

        // when
        let result = p1.distance_to(&p2);

        // then
        assert_eq!(result, 5.0);
    }

    #[test]
    fn point_distance_to_origin() {
        // given
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 0.0, y: -5.0 };

        // when
        let result = p2.distance_to(&p1);

        // then
        assert_eq!(result, 5.0);
    }

    // Vector ////////////////////////////////////////////////////////////////

    #[test]
    fn vector_add_assigns() {
        // given
        let mut sut = Vector { dx: 3.0, dy: 4.0 };

        // when
        sut += Vector { dx: 9.5, dy: -3.5 };

        // then
        assert_eq!(sut, Vector { dx: 12.5, dy: 0.5 });
    }

    #[test]
    fn vector_scalar_multiplies() {
        // given
        let sut = Vector { dx: 3.0, dy: 4.0 };

        // when
        let result = &sut * 3.0;

        // then
        assert_eq!(result, Vector { dx: 9.0, dy: 12.0 })
    }

    #[test]
    fn vector_scalar_divides() {
        // given
        let sut = Vector { dx: 3.0, dy: 12.0 };

        // when
        let result = &sut / 3.0;

        // then
        assert_eq!(result, Vector { dx: 1.0, dy: 4.0 });
    }

    #[test]
    fn vector_inner_product() {
        // given
        let a = Vector { dx: 3.4, dy: -4.9 };
        let b = Vector { dx: 10.0, dy: 6.3 };

        // when
        let result = &a * &b;

        // then
        assert!((result - 3.13).abs() < 0.00001);
    }

    #[test]
    fn vector_magnitude() {
        // given, when, then
        assert_eq!(Vector { dx: 3.0, dy: 4.0 }.magnitude(), 5.0)
    }

    #[test]
    fn vector_normalize() {
        // given
        let sut = Vector { dx: 3.3, dy: 5.2 };

        // when
        match sut.normalized() {
            None => panic!(),
            Some(result) => {
                // then
                assert!((result.magnitude() - 1.0).abs() < 0.0000001);
            }
        };
    }

    #[test]
    fn vector_does_not_normalize_if_zero() {
        // given, when, then
        assert_eq!(Vector::zero().normalized(), None)
    }

    // Size //////////////////////////////////////////////////////////////////

    #[test]
    #[should_panic(expected = "A size's width and/or height must be positive.")]
    fn size_non_positive_width() {
        // given, when , then
        Size::new(-1.0, 1.0);
    }

    #[test]
    #[should_panic(expected = "A size's width and/or height must be positive.")]
    fn size_non_positive_height() {
        // given, when , then
        Size::new(10.0, 0.0);
    }

    // Rect //////////////////////////////////////////////////////////////////

    #[test]
    #[should_panic(expected = "A size's width and/or height must be positive.")]
    fn rect_non_positive_size() {
        // given, when , then
        Rect::new(-1.0, 1.0, -1.0, 0.0);
    }

    #[test]
    fn rect_quadrants() {
        // given
        let sut = Rect::new(0.0, 0.0, 6.0, 8.0);

        // when
        let (nw, ne, sw, se) = sut.quadrants();

        // then
        assert_eq!(nw, Rect::new(0.0, 4.0, 3.0, 4.0));
        assert_eq!(ne, Rect::new(3.0, 4.0, 3.0, 4.0));
        assert_eq!(sw, Rect::new(0.0, 0.0, 3.0, 4.0));
        assert_eq!(se, Rect::new(3.0, 0.0, 3.0, 4.0));
    }

    #[test]
    fn rect_contains_point() {
        // given
        let sut = Rect::new(0.0, 0.0, 10.0, 5.0);

        // then
        assert!(sut.contains(&Point::new(0.0, 0.0)));
        assert!(sut.contains(&Point::new(3.0, 3.0)));
        assert!(sut.contains(&Point::new(10.0, 5.0)));

        assert!(!sut.contains(&Point::new(-0.0001, 0.0)));
        assert!(!sut.contains(&Point::new(10.0000, 5.00001)));
        assert!(!sut.contains(&Point::new(14.0, 5.01)));
    }

//    #[test]
//    fn rect_which_quadrant() {
//        // given
//        let sut = Rect::new(0.0, 0.0, 5.0, 5.0);
//        let (nw, ne, sw, se) = sut.quadrants();
//
//        // then (bottom left of each quadrant)
//        assert_eq!(sut.which_quadrant(&Point::new(0.0, 2.5)), Some((NW, nw.clone())));
//        assert_eq!(sut.which_quadrant(&Point::new(2.5, 2.5)), Some((NW, nw.clone())));
//        assert_eq!(sut.which_quadrant(&Point::new(0.0, 0.0)), Some((SW, sw.clone())));
//        assert_eq!(sut.which_quadrant(&Point::new(2.5, 0.0)), Some((SW, sw.clone())));
//
//        // then (top right of each quadrant)
//        assert_eq!(sut.which_quadrant(&Point::new(2.5, 5.0)), Some((NW, nw.clone())));
//        assert_eq!(sut.which_quadrant(&Point::new(5.0, 5.0)), Some((NE, ne.clone())));
//        assert_eq!(sut.which_quadrant(&Point::new(2.5, 2.5)), Some((NW, nw.clone())));
//        assert_eq!(sut.which_quadrant(&Point::new(5.0, 2.5)), Some((NE, ne.clone())));
//
//        // then (anywhere in quadrant)
//        assert_eq!(sut.which_quadrant(&Point::new(0.3, 2.9)), Some((NW, nw.clone())));
//        assert_eq!(sut.which_quadrant(&Point::new(2.6, 4.2)), Some((NE, ne.clone())));
//        assert_eq!(sut.which_quadrant(&Point::new(1.0, 2.0)), Some((SW, sw.clone())));
//        assert_eq!(sut.which_quadrant(&Point::new(3.7, 2.4)), Some((SE, se.clone())));
//
//        // then
//        assert_eq!(sut.which_quadrant(&Point::new(-2.5, 5.0)), None);
//        assert_eq!(sut.which_quadrant(&Point::new(5.4, 0.4)),  None);
//        assert_eq!(sut.which_quadrant(&Point::new(2.5, -4.0)), None);
//        assert_eq!(sut.which_quadrant(&Point::new(4.0, 6.5)),  None);
//    }
}
