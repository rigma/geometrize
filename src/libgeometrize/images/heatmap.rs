use std::ops::{Add, AddAssign};
use image::{GrayImage, ImageBuffer, Luma};

/// A heatmap is a 2D canvas that shows magnitude of a phenomenon as colors.
/// This data structure is storing the magnitude in a matrix of `u64` words
/// and can be exported as grayscale image encoded on 8-bits or 16-bits.
///
/// # Example
///
/// ```
/// use libgeometrize::images::Heatmap;
///
/// // Instanciating a simple heatmap from a closure
/// let heatmap = Heatmap::from_fn(32, 32, |x, y| {
///    if (x * y) % 2 == 0 {
///         (x * y) as u64
///    } else {
///         0
///    }
/// });
/// ```
#[derive(Clone, Debug)]
pub struct Heatmap {
    inner: Vec<u64>,
    width: u32,
    height: u32,
}

impl Heatmap {
    /// Instanciates a new heatmap with the given dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            inner: vec![0; (width * height) as usize],
            width,
            height,
        }
    }

    /// Instanciates a new heatmap with the given dimensions and fill it with
    /// the values produced by a user-provided closure. This closure is taking
    /// `x` and `y` coordinates as arguments and must return a `u64` word.
    pub fn from_fn<F>(width: u32, height: u32, f: F) -> Self
    where
        F: Fn(u32, u32) -> u64
    {
        let mut inner = vec![0; (width * height) as usize];
        for idx in 0..inner.capacity() {
            let x = idx as u32 % width;
            let y = idx as u32 / width;
            inner[idx] = f(x, y);
        }

        Self {
            inner,
            width,
            height,
        }
    }

    /// Returns the dimensions of the current heatmap.
    #[inline]
    pub const fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Returns the width of the current heatmap.
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the current heatmap.
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Clears the current heatmap.
    pub fn clear(&mut self) {
        self.inner.clear();
        self.inner.shrink_to_fit();
        self.inner = vec![0; (self.width * self.height) as usize];
    }

    /// Retrieves a reference to a pixel contained in the heatmap thanks
    /// to user-supplied coordinates.
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&u64> {
        if x < self.width && y < self.height {
            Some(&self.inner[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    /// Retrieves a mutable reference to a pixel contained in the heatmap
    /// thanks to user-supplied coordinates.
    pub fn get_pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut u64> {
        if x < self.width && y < self.height {
            Some(&mut self.inner[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    /// Instanciates a copy of the current heatmap converted into a 8-bits grayscale
    /// image with a supplied `gamma` factor.
    pub fn to_luma8(&self, gamma: f64) -> GrayImage {
        let max_heat = self.max_heat() as f64;

        GrayImage::from_fn(self.width, self.height, |x, y| {
            let px = self.inner[(y * self.width + x) as usize] as f64 / max_heat;
            let px = px.powf(gamma);
            Luma([(255.0 * px) as u8])
        })
    }

    /// Instanciates a copy of the current heatmap converted into a 16-bits grayscale
    /// image with a supplied `gamma` factor.
    pub fn to_luma16(&self, gamma: f64) -> ImageBuffer<Luma<u16>, Vec<u16>> {
        let max_heat = self.max_heat() as f64;

        ImageBuffer::<Luma<u16>, Vec<u16>>::from_fn(self.width, self.height, |x, y| {
            let px = self.inner[(y * self.width + x) as usize] as f64 / max_heat;
            let px = px.powf(gamma);
            Luma([(255.0 * px) as u16])
        })
    }

    fn max_heat(&self) -> u64 {
        self.inner
            .iter()
            .fold(0, |max, px| {
                if *px > max {
                    *px
                } else {
                    max
                }
            })
    }
}

impl Add for Heatmap {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let width = std::cmp::min(self.width, other.width);
        let height = std::cmp::min(self.height, other.height);

        let mut inner = vec![0;  (width * height) as usize];
        for i in 0..inner.capacity() {
            inner[i] = self.inner[i] + other.inner[i];
        }

        Self {
            inner,
            width,
            height,
        }
    }
}

impl AddAssign for Heatmap {
    fn add_assign(&mut self, other: Self) {
        let width = std::cmp::min(self.width, other.width);
        let height = std::cmp::min(self.height, other.height);

        let mut inner = vec![0;  (width * height) as usize];
        for i in 0..inner.capacity() {
            inner[i] = self.inner[i] + other.inner[i];
        }

        *self = Self {
            inner,
            width,
            height
        };
    }
}

impl Eq for Heatmap {}
impl PartialEq for Heatmap {
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }

        for i in 0..self.inner.len() {
            if self.inner[i] != other.inner[i] {
                return false;
            }
        }

        true
    }
}

impl PartialEq<Vec<u64>> for Heatmap {
    fn eq(&self, other: &Vec<u64>) -> bool {
        if self.inner.len() != other.len() {
            return false;
        }

        for i in 0..self.inner.len() {
            if self.inner[i] != other[i] {
                return false
            }
        }

        true
    }
}

impl PartialEq<Heatmap> for Vec<u64> {
    fn eq(&self, other: &Heatmap) -> bool {
        *other == *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_allocates_a_heatmap_thanks_to_a_closure() {
        let heatmap = Heatmap::from_fn(2, 2, |x, y| x.pow(y) as u64);
        assert_eq!(vec![1u64, 1, 0, 1], heatmap);
    }

    #[test]
    fn it_can_sum_heatmaps() {
        let a = Heatmap::from_fn(2, 2, |x, y| (4 * (x + y)) as u64);
        let b = Heatmap::from_fn(2, 2, |x, y| (x + y) as u64);

        assert_eq!(vec![0u64, 5, 5, 10], a + b);
    }
}
