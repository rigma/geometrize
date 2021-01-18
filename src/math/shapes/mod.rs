mod ellipse;
mod polygon;
mod rectangle;
mod triangle;

/// Defines the common behavior of all mathematical shapes.
pub trait Shape {
    fn mutate(&mut self);

    /// Indicates if the current shape instance is valid or not by a
    /// user-defined constraint. By default, a shape is always valid.
    fn is_valid(&self) -> bool {
        true
    }
}

pub use ellipse::Ellipse;
pub use polygon::Polygon;
pub use rectangle::Rectangle;
pub use triangle::Triangle;
