use crate::geometry::Axis;
use crate::geometry::Bone;
use crate::geometry::Vector;
use crate::unicornify::Legs;
use crate::unicornify::Neck;

#[derive(Debug)]
pub struct Torso {
    pub neck: Neck,
    pub torso: Bone,
    pub tail: Bone,
    pub legs: Legs,
}

impl Torso {
    pub fn new(neck: Neck, torso: Bone, tail: Bone, legs: Legs) -> Self {
        Torso {
            neck,
            torso,
            tail,
            legs,
        }
    }

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) {
        self.neck.rotate_around(other, angle, axis);
        self.torso.rotate_around(other, angle, axis);
        self.tail.rotate_around(other, angle, axis);
        self.legs.rotate_around(other, angle, axis);
    }
}
