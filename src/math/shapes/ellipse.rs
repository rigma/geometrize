/// Defines an ellipse by using its cartesian representation:
/// $\left ( \frac{x - u}{a} \right ) ^ 2 + \left ( \frac{y - v}{b} \right ) ^ 2 = 1$
///
/// This ellipse can be rotated by an angle given in radians if it's provided when
/// the ellipse is instanciated.
///
/// This shape is also implementing the [Shape] trait.
///
/// # Example
///
/// ```
/// // Generating a random non-rotated ellipse
/// let ellipse = Ellipse::random();
///
/// // Generating a rotated ellipse with its parameters
/// let ellipse = Ellipse::new()
///     .u(0.0)
///     .v(0.0)
///     .a(1.0)
///     .b(1.0)
///     .angle(std::f64::consts::PI / 4)
///     .build();
/// ```
///
/// [Shape]: ./Shape.trait.html
#[derive(Clone, Copy, Debug)]
pub struct Ellipse {
    u: f64,
    v: f64,
    a: f64,
    b: f64,
    angle: Option<f64>,
}

impl Ellipse {
    pub fn new() -> EllipseBuilder {
        EllipseBuilder::default()
    }

    /// Instanciates a new random non-rotated ellipse.
    pub fn random() -> Self {
        // TODO: generates the random ellipse thanks to a RNG
        Self {
            u: 0.0,
            v: 0.0,
            a: 1.0,
            b: 1.0,
            angle: None,
        }
    }

    /// Instanciates a new random rotated ellipse.
    pub fn random_rotated() -> Self {
        // TODO: generates a random rotated ellipse thanks to a RNG
        Self {
            u: 0.0,
            v: 0.0,
            a: 1.0,
            b: 1.0,
            angle: Some(0.0),
        }
    }

    /// Indicates if the current ellipse is a circle by checking that its half-heights
    /// `a` and `b` are equals.
    pub fn is_circle(&self) -> bool {
        // Since `a` and `b` are floating point numbers, we cannot be sure that they are
        // strictly equals. We'll consider them equals if their difference is strictly
        // lesser than `f64::EPSILON`
        self.a - self.b < f64::EPSILON
    }

    /// Indicates if the current elippse is rotated or not.
    pub const fn is_rotated(&self) -> bool {
        self.angle.is_some()
    }
}

impl super::Shape for Ellipse {
    fn mutate(&mut self) {
        //
    }
}

#[derive(Default)]
pub struct EllipseBuilder {
    u: f64,
    v: f64,
    a: f64,
    b: f64,
    angle: Option<f64>,
}

impl EllipseBuilder {
    pub fn u(mut self, u: f64) -> Self {
        self.u = u;
        self
    }

    pub fn v(mut self, v: f64) -> Self {
        self.v = v;
        self
    }

    pub fn a(mut self, a: f64) -> Self {
        self.a = a;
        self
    }

    pub fn b(mut self, b: f64) -> Self {
        self.b = b;
        self
    }

    pub fn angle(mut self, angle: f64) -> Self {
        self.angle = Some(angle);
        self
    }

    pub fn build(self) -> Ellipse {
        Ellipse {
            u: self.u,
            v: self.v,
            a: self.a,
            b: self.b,
            angle: self.angle,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_indicates_if_an_ellipse_is_a_circle() {
        let circle = Ellipse {
            u: 0.0,
            v: 0.0,
            a: 1.0,
            b: 1.0,
            angle: None,
        };

        assert!(circle.is_circle())
    }

    #[test]
    fn it_indicates_if_an_ellipse_is_rotated() {
        let ellipse = Ellipse {
            u: 0.0,
            v: 0.0,
            a: 1.0,
            b: 2.0,
            angle: Some(std::f64::consts::PI / 4.0),
        };

        assert!(ellipse.is_rotated());
    }
}
