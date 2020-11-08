mod avatar;
mod background;
mod grass;
mod head;
mod leg;
mod legs;
mod mane;
mod neck;
mod pose;
mod steak;
mod torso;
mod unicorn;
mod unicorn_data;

pub use background::Background;
pub use grass::Grass;
pub use head::Head;
pub use leg::Leg;
pub use legs::Legs;
pub use mane::Mane;
pub use neck::Neck;
pub use pose::Pose;
pub use steak::Steak;
pub use torso::Torso;
pub use unicorn::Unicorn;
pub use unicorn_data::UnicornData;

use std::f64::consts::PI;
const DEGREE: f64 = PI / 180f64;
