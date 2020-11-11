use crate::geometry::Axis;
use crate::geometry::Ball;
use crate::geometry::Bone;
use crate::geometry::Vector;
use crate::render::GroupTracer;
use crate::render::WorldView;

#[derive(Clone, Debug)]
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

    pub fn add_traceable(&self, mut tracer: &mut GroupTracer, world_view: WorldView) {
        self.calf.add_traceable(&mut tracer, world_view.clone());
        self.shin.add_traceable(&mut tracer, world_view);
    }

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) {
        self.hip.rotate_around(other, angle, axis);
        self.knee.rotate_around(other, angle, axis);
        self.hoof.rotate_around(other, angle, axis);
    }
}
