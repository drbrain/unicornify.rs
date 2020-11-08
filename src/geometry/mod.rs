mod axis;
mod ball;
mod bone;
mod gamma;
mod point;
mod vector;

pub use axis::Axis;
pub use ball::Ball;
pub use bone::Bone;
pub use gamma::Gamma;
pub use point::Point;
pub use vector::Vector;

use std::f64::consts::PI;
pub const DEGREE: f64 = PI / 180f64;
