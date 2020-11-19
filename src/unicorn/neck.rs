use crate::geometry::Axis;
use crate::geometry::Bone;
use crate::geometry::Vector;
use crate::render::GroupTracer;
use crate::render::WorldView;
use crate::unicorn::Head;
use crate::unicorn::Mane;

#[derive(Clone, Debug)]
pub struct Neck {
    pub head: Head,
    pub neck: Bone,
    pub mane: Mane,
}

impl Neck {
    pub fn new(head: Head, neck: Bone, mane: Mane) -> Self {
        Neck { head, neck, mane }
    }

    pub fn add_traceable(&self, mut tracer: &mut GroupTracer, world_view: &WorldView) {
        self.head.add_traceable(&mut tracer, world_view);
        self.neck.add_traceable(&mut tracer, world_view);
        self.mane.add_traceable(&mut tracer, world_view);
    }

    /// Rotates the neck (head, mane, and neck) around +other+, taking care not to rotate any
    /// joints twice.
    pub fn rotate_around(&self, other: &Vector, angle: f64, axis: Axis) {
        self.head.rotate_around(other, angle, axis);
        // b1 is the "head" end of the neck bone which was rotated in the line above
        self.neck.b2.rotate_around(other, angle, axis);
        self.mane.rotate_around(other, angle, axis);
    }
}
