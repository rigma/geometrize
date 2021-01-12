mod ellipse;
mod rectangle;
mod triangle;

/// Defines the common behavior of all mathematical shapes.
pub trait Shape {
    fn mutate(&mut self);
}

pub use ellipse::Ellipse;
pub use rectangle::Rectangle;
pub use triangle::Triangle;
