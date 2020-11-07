mod avatar;
mod background;
mod ball;
mod bone;
mod figure;
mod grass;
mod leg;
mod legs;
mod pose;
mod steak;
mod thing;
mod unicorn;
mod unicorn_data;

pub use background::Background;
pub use ball::Ball;
pub use bone::Bone;
pub use figure::Figure;
pub use grass::Grass;
pub use leg::Leg;
pub use legs::Legs;
pub use pose::Pose;
pub use steak::Steak;
pub use thing::Thing;
pub use unicorn::Unicorn;
pub use unicorn_data::UnicornData;

use std::f64::consts::PI;
const DEGREE: f64 = PI / 180f64;
