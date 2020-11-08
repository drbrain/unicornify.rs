use crate::geometry::Axis;
use crate::geometry::Vector;
use crate::unicorn::Leg;

#[derive(Clone, Debug)]
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

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) {
        self.fr.rotate_around(other, angle, axis);
        self.fl.rotate_around(other, angle, axis);
        self.br.rotate_around(other, angle, axis);
        self.bl.rotate_around(other, angle, axis);
    }
}
