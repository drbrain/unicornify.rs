use crate::render::BallProjection;

use image::RgbaImage;

use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::convert::From;

#[derive(Clone, Debug)]
pub struct Bounds {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub z_min: f64,
    pub z_max: f64,
    pub empty: bool,
}

impl Bounds {
    pub fn empty() -> Self {
        Bounds {
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
            z_min: 0.0,
            z_max: 0.0,
            empty: true,
        }
    }

    pub fn for_ball(bp: &BallProjection) -> Self {
        let (x, y) = if bp.center_cs().z < 0.0 {
            (bp.center_cs().x, bp.center_cs().y)
        } else {
            (bp.x(), bp.y())
        };

        let x_min = x - bp.projected_radius();
        let x_max = x + bp.projected_radius();
        let y_min = y - bp.projected_radius();
        let y_max = y + bp.projected_radius();
        let z_min = bp.center_cs().z - bp.base.radius;
        let z_max = bp.z() + bp.base.radius;
        let empty = false;

        Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            empty,
        }
    }

    pub fn for_balls(bps: Vec<BallProjection>) -> Self {
        bps.iter()
            .map(|bp| Bounds::for_ball(bp))
            .fold(Bounds::empty(), |a, b| a.union(&b))
    }

    pub fn dx(&self) -> f64 {
        if self.empty {
            0.0
        } else {
            self.x_max - self.x_min
        }
    }

    pub fn dy(&self) -> f64 {
        if self.empty {
            0.0
        } else {
            self.y_max - self.y_min
        }
    }

    pub fn intersection(&self, other: &Bounds) -> Self {
        if self.empty && other.empty {
            return Bounds::empty();
        }

        let x_min = self.x_min.max(other.x_min);
        let x_max = self.x_max.min(other.x_max);
        let y_min = self.y_min.max(other.y_min);
        let y_max = self.y_max.min(other.y_max);
        let z_min = self.z_min.max(other.z_min);
        let z_max = self.z_max.min(other.z_max);
        let empty = false;

        if x_min > x_max || y_min > y_max || z_min > z_max {
            return Bounds::empty();
        }

        Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            empty,
        }
    }

    pub fn union(&self, other: &Bounds) -> Self {
        match (self.empty, other.empty) {
            (true, true) => return Bounds::empty(),
            (false, true) => return Bounds::empty(),
            (true, false) => return other.clone(),
            (false, false) => (),
        }

        let x_min = self.x_min.min(other.x_min);
        let x_max = self.x_max.max(other.x_max);
        let y_min = self.y_min.min(other.y_min);
        let y_max = self.y_max.max(other.y_max);
        let z_min = self.z_min.min(other.z_min);
        let z_max = self.z_max.max(other.z_max);
        let empty = false;

        Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            empty,
        }
    }
}

impl From<&mut RgbaImage> for Bounds {
    fn from(image_buffer: &mut RgbaImage) -> Bounds {
        let x_min = 0.0;
        let x_max = image_buffer.width() as f64;
        let y_min = 0.0;
        let y_max = image_buffer.height() as f64;
        let z_min = 0.0;
        let z_max = 0.0;
        let empty = false;

        Bounds {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            empty,
        }
    }
}

impl Eq for Bounds {}

impl Ord for Bounds {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z_min.partial_cmp(&other.z_min).unwrap()
    }
}

impl PartialEq for Bounds {
    fn eq(&self, other: &Self) -> bool {
        self.z_min == other.z_min
    }
}

impl PartialOrd for Bounds {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.z_min.partial_cmp(&other.z_min)
    }
}
