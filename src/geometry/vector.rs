use crate::geometry::Axis;

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn reverse(&self, axis: &Axis) -> Vector {
        match axis {
            Axis::X => Vector::new(self.z, self.x, self.y),
            Axis::Y => Vector::new(self.x, self.z, self.y),
            Axis::Z => self.clone(),
        }
    }

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) -> Vector {
        let shifted = (*self - other).swap(&axis);

        let x = shifted.x * angle.cos() - shifted.y * angle.sin();
        let y = shifted.x * angle.sin() - shifted.y * angle.cos();
        let z = shifted.z;

        Vector::new(x, y, z).reverse(&axis) + other
    }

    pub fn swap(&self, axis: &Axis) -> Vector {
        match axis {
            Axis::X => Vector::new(self.y, self.z, self.x),
            Axis::Y => Vector::new(self.x, self.z, self.y),
            Axis::Z => self.clone(),
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
