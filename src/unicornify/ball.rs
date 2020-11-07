use crate::Axis;
use crate::Color;
use crate::Vector;

use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, Debug, PartialEq)]
pub struct Ball {
    pub center: Vector,
    pub radius: f64,
    pub color: Color,
}

impl Ball {
    pub fn new(x: f64, y: f64, z: f64, radius: f64, color: Color) -> Self {
        let center = Vector::new(x, y, z);

        Ball {
            center,
            radius,
            color,
        }
    }

    pub fn new_v(center: Vector, radius: f64, color: Color) -> Self {
        Ball {
            center,
            radius,
            color,
        }
    }

    pub fn move_to_sphere(&self, other: Ball) -> Ball {
        self.set_distance(other.radius, other)
    }

    pub fn set_distance(&self, distance: f64, other: Ball) -> Ball {
        let span = self.center - other.center;
        let center = other.center + span * distance / span.length();

        Ball::new_v(center, self.radius, self.color.clone())
    }

    pub fn set_gap(&self, gap: f64, other: Ball) -> Ball {
        self.set_distance(self.radius + other.radius + gap, other)
    }

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) -> Ball {
        let new_center = self.center.rotate_around(other, angle, axis);

        Ball::new_v(new_center, self.radius, self.color.clone())
    }
}

// TODO is PartialEq enough for this to be right with respect to f64?
impl Eq for Ball {}

impl Hash for Ball {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        let radius_val = format!("{:.10e}", self.radius);
        radius_val.hash(state);
        self.color.hash(state);
    }
}
