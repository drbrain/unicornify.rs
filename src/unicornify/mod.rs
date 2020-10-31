mod avatar;
mod background;
mod grass;
mod pose;
mod unicorn;
mod vector;

pub use background::Background;
pub use grass::Grass;
pub use pose::Pose;
pub use unicorn::Unicorn;
pub use vector::Vector;

use std::f64::consts::PI;
const DEGREE: f64 = PI / 180f64;
