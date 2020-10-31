use crate::pyrand::Random;

pub enum Pose {
    RotaryGallop { phase: f64 },
    Walk { phase: f64 },
}

impl Pose {
    pub fn new(rand: &mut Random) -> Pose {
        let kind = rand.choice(2);
        let phase = rand.rand();

        match kind {
            0 => Pose::RotaryGallop { phase },
            1 => Pose::Walk { phase },
            _ => unreachable!(),
        }
    }
}
