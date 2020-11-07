pub use crate::unicornify::Ball;
pub use crate::unicornify::Bone;

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
}
