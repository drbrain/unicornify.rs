use crate::geometry::Vector;

pub struct WorldView {
    camera_positon: Vector,
    look_at_point: Vector,
    focal_length: f64,
    ux: Vector,
    uy: Vector,
    zero: Vector,
}

impl WorldView {
    pub fn new(camera_positon: Vector, look_at_point: Vector, focal_length: f64) -> Self {
        let view = look_at_point - camera_positon;
        let n = view * 1.0 / view.length();

        let (ux, uy) = n.cross_axes();
        let zero = camera_positon + ((look_at_point - camera_positon).unit() * focal_length);

        WorldView {
            camera_positon,
            look_at_point,
            focal_length,
            ux,
            uy,
            zero,
        }
    }
}
