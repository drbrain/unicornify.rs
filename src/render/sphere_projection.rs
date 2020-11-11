use crate::render::WorldView;
use crate::geometry::Vector;
use crate::geometry::intersect_plane_line;

#[derive(Clone, Debug)]
pub struct SphereProjection {
    pub center_cs: Vector,
    projected_center_cs: Vector,
    projected_center_os: Vector,
    pub projected_radius: f64,
    world_view: WorldView,
}

impl SphereProjection {
    pub fn new(world_view: WorldView, center: Vector, radius: f64) -> Self {
        let cam2c = center - world_view.camera_position;
        let dist = cam2c.length();

        let intf = match intersect_plane_line(world_view.zero, world_view.ux, world_view.uy,
            world_view.camera_position, cam2c) {
            Some(v) => v,
            None => todo!("Unable to project sphere"),
        };

        let projected_center_os = world_view.camera_position + cam2c * intf.z;
        let mut projected_center_cs = Vector::new(intf.x, intf.y, world_view.focal_length);

        let dir = if intf.z < 0.0 { -1.0 } else { 1.0 };

        let center_cs = projected_center_cs * (dir * dist / projected_center_cs.length());

        if intf.z < 0.0 {
            projected_center_cs.x *= -1.0;
            projected_center_cs.y *= -1.0;
        }

        if radius == 0.0 {
            return SphereProjection {
                center_cs,
                projected_center_cs,
                projected_center_os,
                projected_radius: 0.0,
                world_view
            };
        }

        let closest_to_cam = world_view.camera_position + cam2c * (1.0 - radius / dist);

        let (u1, u2) = (cam2c * (1.0 / dist)).cross_axes();

        let mut r: f64 = 0.0;

        for c1 in [-1.0f64, 1.0f64].iter() {
            for c2 in [-1.0f64, 1.0f64].iter() {
                let p = closest_to_cam + u1 * *c1 * radius + u2 * *c2 * radius;

                let pr = SphereProjection::new(world_view.clone(), p, 0.0);
                r = r.max(pr.x() - projected_center_cs.x);
                r = r.max(pr.y() - projected_center_cs.y);
            }
        }

        let projected_radius = r;

        SphereProjection {
                center_cs,
                projected_center_cs,
                projected_center_os,
                projected_radius,
                world_view,
        }
    }

    pub fn x(&self) -> f64 {
        self.projected_center_cs.x
    }

    pub fn y(&self) -> f64 {
        self.projected_center_cs.y
    }

    pub fn z(&self) -> f64 {
        self.center_cs.length()
    }
}
