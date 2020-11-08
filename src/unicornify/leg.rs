pub use crate::unicornify::Ball;
pub use crate::unicornify::Bone;
use crate::Axis;
use crate::Vector;

#[derive(Debug)]
pub struct Leg {
    pub hip: Ball,
    pub knee: Ball,
    pub hoof: Ball,
    pub calf: Bone,
    pub shin: Bone,
}

impl Leg {
    pub fn new(hip: Ball, knee: Ball, hoof: Ball) -> Leg {
        let calf = Bone::new(hip.clone(), knee.clone());
        let shin = Bone::new(knee.clone(), hoof.clone());

        Leg {
            hip,
            knee,
            hoof,
            calf,
            shin,
        }
    }

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) {
        self.hip.rotate_around(other, angle, axis);
        self.knee.rotate_around(other, angle, axis);
        self.hoof.rotate_around(other, angle, axis);
    }
}
