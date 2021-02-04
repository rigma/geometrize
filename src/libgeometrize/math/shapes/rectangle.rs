use super::Shape;
use crate::math::Point;

const MAX_ASPECT_RATIO: f64 = 5.0;

/// Defines a rectangle shape by using an origin point. This origin point is then
/// used to defines a unit square that we'll scale to the width and height of the
/// desired rectangle. This rectangle can also be rotated by an angle given in
/// radians.
///
/// # Example
///
/// ```
/// use libgeometrize::math::shapes::{Rectangle, Shape};
///
/// // Instanciates a rotated golden rectangle.
/// let rect = Rectangle::new()
///     .height((1.0 + 5_f64.sqrt()) / 2.0)
///     .angle(std::f64::consts::FRAC_PI_4)
///     .build();
///
/// // Check that the instanciated rectangle is a valid one.
/// assert!(rect.is_valid());
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    /// The origin of the rectangle.
    pub origin: Point,
    scaling: (f64, f64),

    /// The value of the rotation angle of the rectangle.
    pub angle: f64,
}

impl Rectangle {
    /// Returns a rectangle builder to instanciates a new rectangle.
    pub fn new() -> RectangleBuilder {
        RectangleBuilder::default()
    }

    /// Returns the width of the rectangle.
    pub const fn width(&self) -> f64 {
        self.scaling.0
    }

    /// Returns the height of the rectangle.
    pub const fn height(&self) -> f64 {
        self.scaling.1
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            origin: Point::zero(),
            scaling: (1.0, 1.0),
            angle: 0.0,
        }
    }
}

impl Shape for Rectangle {
    fn mutate(&mut self) {
        //
    }

    fn is_valid(&self) -> bool {
        let (width, height) = if self.scaling.0 < self.scaling.1 {
            (self.scaling.1, self.scaling.0)
        } else {
            self.scaling
        };

        width / height <= MAX_ASPECT_RATIO
    }
}

#[derive(Debug)]
pub struct RectangleBuilder {
    origin: (f64, f64),
    scaling: (f64, f64),
    angle: f64,
}

impl RectangleBuilder {
    /// Defines the origin of the new rectangle.
    pub fn origin(mut self, x: f64, y: f64) -> Self {
        self.origin = (x, y);
        self
    }

    /// Defines the general aspect of the new rectangle (width and height).
    pub fn aspect(mut self, width: f64, height: f64) -> Self {
        self.scaling = (width, height);
        self
    }

    /// Defines the width of the new rectangle.
    pub fn width(mut self, width: f64) -> Self {
        self.scaling.0 = width;
        self
    }

    /// Defines the height of the new rectangle.
    pub fn height(mut self, height: f64) -> Self {
        self.scaling.1 = height;
        self
    }

    /// Defines the rotation angle of the new rectangle.
    pub fn angle(mut self, angle: f64) -> Self {
        self.angle = angle;
        self
    }

    /// Returns a new instance of a rectangle shape.
    pub fn build(self) -> Rectangle {
        Rectangle {
            origin: Point::new(self.origin.0, self.origin.1),
            scaling: self.scaling,
            angle: self.angle,
        }
    }
}

impl Default for RectangleBuilder {
    fn default() -> Self {
        Self {
            origin: (0.0, 0.0),
            scaling: (1.0, 1.0),
            angle: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_a_rectangle() {
        let r = Rectangle::new().origin(0.0, 0.0).aspect(25.0, 5.0).build();

        assert!(r.is_valid());
    }

    #[test]
    fn it_invalidates_a_rectangle() {
        let r = Rectangle::new().origin(0.0, 0.0).aspect(30.0, 5.0).build();

        assert!(!r.is_valid());
    }
}
