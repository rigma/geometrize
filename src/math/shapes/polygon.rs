use crate::math::{Point, Vector};
use super::Shape;

/// Defines a polygon shape thanks to a vector of points defining
/// its vertices. This shape can be validated by using [`is_valid`]
/// method that we'll check that the polygon is convex. If the current
/// polygon is not degenerated and convex, then we'll validate it.
#[derive(Clone, Debug, Default)]
pub struct Polygon {
    vertices: Vec<Point>
}

impl Polygon {
    /// Instanciates a new polygon shape from a vector of points.
    pub fn new(vertices: Vec<Point>) -> Self {
        Self { vertices }
    }

    /// Returns the order of the current polygon.
    #[inline]
    pub fn order(&self) -> usize {
        self.vertices.len()
    }

    /// Checks if the current polygon is valid or not. To do so, the
    /// method we'll check that the polygon is not dengenerated or not
    /// convex by checking that the cross products of all its vertices
    /// have the same sign.
    pub fn is_valid(&self) -> bool {
        let order = self.order();
        if order < 3 {
            return false;
        }

        let sign = {
            let u: Vector = self.vertices[1] - self.vertices[0];
            let v: Vector = self.vertices[2] - self.vertices[1];
            u.cross(&v) > 0.0
        };

        for idx in 1..order {
            let (u, v): (Vector, Vector) = {
                let i = idx % order;
                let j = (idx + 1) % order;
                let k = (idx + 2) % order;
                (self.vertices[j] - self.vertices[i], self.vertices[k] - self.vertices[j])
            };

            if (u.cross(&v) > 0.0) != sign {
                return false;
            }
        }

        true
    }
}

impl From<&[Point]> for Polygon {
    fn from(vertices: &[Point]) -> Self {
        Self::new(Vec::from(vertices))
    }
}

impl From<Vec<Point>> for Polygon {
    fn from(vertices: Vec<Point>) -> Self {
        Self { vertices }
    }
}

impl Shape for Polygon {
    fn mutate(&mut self) {
        //
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_a_polygon() {
        let polygon = Polygon::from(vec![
            Point::zero(),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0)
        ]);

        assert!(polygon.is_valid());
    }

    #[test]
    fn it_invalidates_a_polygon() {
        let polygon = Polygon::from(vec![
            Point::zero(),
            Point::new(0.0, 1.0),
            Point::new(10.0, 10.0),
            Point::new(1.0, 1.0),
            Point::new(1.0, 0.0)
        ]);

        assert!(!polygon.is_valid());
    }
}
