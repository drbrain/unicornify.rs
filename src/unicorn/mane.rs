use crate::geometry::Axis;
use crate::geometry::Bone;
use crate::geometry::Vector;

#[derive(Clone, Debug)]
pub struct Mane {
    pub mane: Vec<Bone>,
}

impl Mane {
    pub fn new(capacity: usize) -> Self {
        let mane: Vec<Bone> = Vec::with_capacity(capacity);

        Mane { mane }
    }

    pub fn push(&mut self, hair: Bone) {
        self.mane.push(hair);
    }

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) {
        for hair in self.mane.iter() {
            hair.rotate_around(other, angle, axis);
        }
    }
}
