use crate::Axis;
use crate::Vector;

use crate::unicornify::Ball;
use crate::unicornify::Bone;
use crate::unicornify::Figure;
use crate::unicornify::Steak;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Thing {
    BallT(Ball),
    BoneT(Bone),
    FigureT(Figure),
    SteakT(Steak),
}

impl Thing {
    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) -> Option<Thing> {
        match self {
            Thing::BallT(b) => Some(Thing::BallT(b.rotate_around(other, angle, axis))),
            _ => None,
        }
    }
}
