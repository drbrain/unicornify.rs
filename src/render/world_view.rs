use crate::geometry::Vector;

#[derive(Clone, Debug)]
pub struct WorldView {
    pub camera_position: Vector,
    pub look_at_point: Vector,
    pub focal_length: f64,
    pub ux: Vector,
    pub uy: Vector,
    pub zero: Vector,
}

impl WorldView {
    pub fn new(camera_position: Vector, look_at_point: Vector, focal_length: f64) -> Self {
        let view = look_at_point - camera_position;
        let n = view * 1.0 / view.length();

        let (ux, uy) = n.cross_axes();
        let zero = camera_position + ((look_at_point - camera_position).unit() * focal_length);

        WorldView {
            camera_position,
            look_at_point,
            focal_length,
            ux,
            uy,
            zero,
        }
    }

    pub fn ray(&self, x: f64, y: f64) -> Vector {
        Vector::new(x, y, self.focal_length).unit()
    }
}
