use crate::unicornify::Leg;

pub struct Legs {
    pub fr: Leg,
    pub fl: Leg,
    pub br: Leg,
    pub bl: Leg,
}

impl Legs {
    pub fn new(fr: Leg, fl: Leg, br: Leg, bl: Leg) -> Self {
        Legs { fr, fl, br, bl }
    }
}
