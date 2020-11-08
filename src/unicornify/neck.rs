use crate::geometry::Axis;
use crate::geometry::Bone;
use crate::geometry::Vector;
use crate::unicornify::Head;
use crate::unicornify::Mane;

#[derive(Debug)]
pub struct Neck {
    pub head: Head,
    pub neck: Bone,
    pub mane: Mane,
}

impl Neck {
    pub fn new(head: Head, neck: Bone, mane: Mane) -> Self {
        Neck { head, neck, mane }
    }

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) {
        self.head.rotate_around(other, angle, axis);
        self.neck.rotate_around(other, angle, axis);
        self.mane.rotate_around(other, angle, axis);
    }
}
