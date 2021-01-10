use crate::math::{Point, Vector};
use super::Shape;

/// Defines a triangle with a vector of 3 vertices which are 3 points on the
/// the plane.
///
/// In `geometrize`, a triangle is considered as valid if all its inner angles
/// are greater than 15°. If not, the triangle is considered as invalid though
/// its still a triangle.
#[derive(Clone, Copy, Default, Debug)]
pub struct Triangle {
    vertices: [Point; 3],
}

/// The minimal value that an internal angle can take in our program.
const MIN_INTERNAL_ANGLE: f64 = 15.0;

impl Triangle {
    /// Instanciates a new triangle from three points of the plane.
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self {
            vertices: [a, b, c],
        }
    }

    // TODO: add a method to instanciate a random triangle

    /// Checks if the current triangle is valid with the constraints we use for
    /// the shape definition.
    pub fn is_valid(&self) -> bool {
        let a1 = {
            let u: Vector = (self.vertices[1] - self.vertices[0]).normalize();
            let v: Vector = (self.vertices[2] - self.vertices[0]).normalize();
            u.dot(&v).acos().to_degrees()
        };
        let a2 = {
            let u: Vector = (self.vertices[0] - self.vertices[1]).normalize();
            let v: Vector = (self.vertices[2] - self.vertices[1]).normalize();
            u.dot(&v).acos().to_degrees()
        };
        let a3 = 180.0 - a2 - a1;

        a1 >= MIN_INTERNAL_ANGLE && a2 >= MIN_INTERNAL_ANGLE && a3 >= MIN_INTERNAL_ANGLE
    }
}

impl From<[Point; 3]> for Triangle {
    fn from(vertices: [Point; 3]) -> Self {
        Self { vertices }
    }
}

impl Shape for Triangle {
    fn mutate(&mut self) {
        //
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validate_a_triangle() {
        let a = Triangle {
            vertices: [Point::zero(), Point::new(0.5, 1.0), Point::new(1.0, 0.0)],
        };
        let b = Triangle {
            vertices: [Point::zero(), Point::new(0.0, 1.0), Point::new(50.0, 0.0)],
        };

        assert!(a.is_valid());
        assert!(!b.is_valid());
    }
}
