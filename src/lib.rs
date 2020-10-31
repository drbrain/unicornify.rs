mod color;
mod point;
mod pyrand;
pub mod unicornify;

pub use color::Color;
pub use point::Point;
pub use pyrand::Random;

#[cfg(test)]
mod test_pyrand;
