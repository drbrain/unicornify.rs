use crate::geometry::Axis;
use crate::geometry::Bone;
use crate::geometry::Vector;
use crate::render::GroupTracer;
use crate::render::WorldView;
use crate::unicorn::Legs;
use crate::unicorn::Neck;

#[derive(Clone, Debug)]
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

    pub fn add_traceable(&self, mut tracer: &mut GroupTracer, world_view: WorldView) {
        self.neck.add_traceable(&mut tracer, world_view.clone());
        self.tail.add_traceable(&mut tracer, world_view.clone());
        self.torso.add_traceable(&mut tracer, world_view.clone());
        self.legs.add_traceable(&mut tracer, world_view);
    }

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) {
        self.neck.rotate_around(other, angle, axis);
        self.torso.rotate_around(other, angle, axis);
        self.tail.rotate_around(other, angle, axis);
        self.legs.rotate_around(other, angle, axis);
    }
}
