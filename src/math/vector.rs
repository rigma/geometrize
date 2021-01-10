#[derive(Clone, Copy, Debug, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

/// The infamous Quake 3 inverse square root function to have a quick
/// approximation of the inverse square root of a floating point number.
///
/// With this implementation, we have $\frac{1}{\sqrt x} - q_rsrt(x) < \epsilon$.
fn q_rsqrt(x: f64) -> f64 {
    // Step 1: evil floating point bits hacking. Here we retrieve the bit
    // representation of the 64-bits IEEE 754 floating point number.
    let word = x.to_bits();

    // Step 2: what the fuck? The magic happens here. We leverage the
    // floating point approximation to get a first approximation of
    // the inverse square root of `x`.
    // Further details here: https://timmmm.github.io/fast-inverse-square-root/
    let word = 0x5fe6eb50c7b537a9 - (word >> 1);
    
    // Step 3: Newton-Raphson iteration. We use this method to have a
    // closer approximation of the inverse square root value.
    let threehalfs = 1.5;
    let x2 = 0.5 * x;
    let y = f64::from_bits(word);

    // We iterate 4 times to have an approximation close to the "real" value
    let y = y * (threehalfs - (x2 * y * y));
    let y = y * (threehalfs - (x2 * y * y));
    let y = y * (threehalfs - (x2 * y * y));
    y * (threehalfs - (x2 * y * y))
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = q_rsqrt(self.dot(self));

        Self {
            x: self.x * magnitude,
            y: self.y * magnitude,
        }
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_rsqrt_is_computing_a_correct_approximation() {
        let target = 0.48795003647426655;

        assert!((target - q_rsqrt(4.2)).abs() < f64::EPSILON);
    }

    #[test]
    fn vector_can_be_normalized() {
        let u = Vector::new(1.0, 1.0);
        let v = u.normalize();

        assert!((v.magnitude() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn vector_can_have_a_dot_product() {
        let u = Vector::new(1.0, 0.0);
        let v = Vector::new(0.0, 1.0);

        assert_eq!(0.0, u.dot(&v));
    }
}
