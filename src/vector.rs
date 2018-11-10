use point::Point;
use std::ops::{ Add, AddAssign, Mul, Div };

/**
 *  Vector in 2D space.
 */
#[derive(Debug)]
#[derive(PartialOrd, PartialEq)]
pub struct Vector {
    pub dx: f64,
    pub dy: f64,
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

impl<'a> Mul<f64> for &'a Vector {
    type Output = Vector;

    /**
     *  Scalar multiplication.
     */
    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

// TODO: Needs testing
impl<'a> Mul for &'a Vector {
    type Output = f64;

    /**
     *  Inner product.
     */
    fn mul(self, rhs: &Vector) -> Self::Output {
        self.dx * rhs.dx + self.dy * rhs.dy
    }
}

impl<'a> Div<f64> for &'a Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            dx: self.dx / rhs,
            dy: self.dy / rhs,
        }
    }
}

impl Vector {
    /**
     *  The zero vector.
     */
    pub fn zero() -> Vector {
        Vector { dx: 0.0, dy: 0.0 }
    }

    /**
     *  The difference vector between two points.
     */
    pub fn difference(lhs: &Point, rhs: &Point) -> Vector {
        Vector {
            dx: (lhs.x - rhs.x) as f64,
            dy: (lhs.y - rhs.y) as f64,
        }
    }
}

impl Vector {
    /**
     *  The magnitude of a vector is the length of its hypotenuse.
     */
    pub fn magnitude(&self) -> f64 {
        ((self.dx * self.dx + self.dy * self.dy) as f64).sqrt()
    }
    
    /**
     *  Normalized copy of self.
     */
    pub fn normalized(&self) -> Option<Vector> {
        if self == &Vector::zero() { return None }
        let magnitude = self.magnitude();

        Some(Vector {
            dx: self.dx / magnitude,
            dy: self.dy / magnitude
        })
    }
}
