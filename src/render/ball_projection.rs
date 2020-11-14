use crate::geometry::Ball;
use crate::geometry::Vector;
use crate::render::SphereProjection;
use crate::render::WorldView;

#[derive(Clone, Debug, PartialEq)]
pub struct BallProjection {
    pub sphere: SphereProjection,
    pub base: Ball,
}

impl BallProjection {
    pub fn new(world_view: &WorldView, base: Ball) -> Self {
        let sphere = SphereProjection::new(world_view, *base.center.borrow(), base.radius);

        BallProjection { sphere, base }
    }

    pub fn center_cs(&self) -> Vector {
        self.sphere.center_cs
    }

    pub fn projected_radius(&self) -> f64 {
        self.sphere.projected_radius
    }

    pub fn x(&self) -> f64 {
        self.sphere.x()
    }

    pub fn y(&self) -> f64 {
        self.sphere.y()
    }

    pub fn z(&self) -> f64 {
        self.sphere.z()
    }
}
