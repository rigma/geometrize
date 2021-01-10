mod ellipse;

/// Defines the common behavior of all mathematical shapes.
pub trait Shape {
    fn mutate(&mut self);
}

pub use ellipse::Ellipse;
