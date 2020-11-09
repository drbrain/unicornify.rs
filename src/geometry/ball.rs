use crate::geometry::Axis;
use crate::geometry::Vector;
use crate::Color;

use std::cell::RefCell;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Ball {
    pub name: String,
    pub center: Rc<RefCell<Vector>>,
    pub radius: f64,
    pub color: Color,
}

impl Ball {
    pub fn new(name: String, x: f64, y: f64, z: f64, radius: f64, color: Color) -> Self {
        let center = Rc::new(RefCell::new(Vector::new(x, y, z)));

        Ball {
            name,
            center,
            radius,
            color,
        }
    }

    pub fn new_v(name: String, center: Vector, radius: f64, color: Color) -> Self {
        let center = Rc::new(RefCell::new(center));

        Ball {
            name,
            center,
            radius,
            color,
        }
    }

    pub fn move_to_sphere(&self, other: Ball) {
        self.set_distance(other.radius, other);
    }

    pub fn set_distance(&self, distance: f64, other: Ball) {
        let span = *self.center.borrow() - *other.center.borrow();
        let new_center = *other.center.borrow() + span * distance / span.length();

        self.center.replace(new_center);
    }

    pub fn set_gap(&self, gap: f64, other: Ball) {
        self.set_distance(self.radius + other.radius + gap, other);
    }

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) {
        let center = self.center.borrow_mut();

        let new_center = center.rotate_around(other, angle, axis);

        self.center.replace(new_center);
    }
}

impl Add<Vector> for Ball {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        *self.center.borrow() + rhs
    }
}

impl Div<f64> for Ball {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        *self.center.borrow() / rhs
    }
}

impl Mul<f64> for Ball {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        *self.center.borrow() * rhs
    }
}

impl Sub<Ball> for Ball {
    type Output = Vector;

    fn sub(self, rhs: Ball) -> Vector {
        *self.center.borrow() - *rhs.center.borrow()
    }
}

impl Sub<Vector> for Ball {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        *self.center.borrow() - rhs
    }
}
