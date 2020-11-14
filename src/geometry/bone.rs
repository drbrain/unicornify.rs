use crate::geometry::Axis;
use crate::geometry::Ball;
use crate::geometry::Gamma;
use crate::geometry::Vector;
use crate::render::BallProjection;
use crate::render::BoneTracer;
use crate::render::GroupTracer;
use crate::render::Tracer;
use crate::render::WorldView;

use std::cell::RefCell;

const DEFAULT_SHADING: f64 = 0.25;

#[derive(Clone, Debug)]
pub struct Bone {
    pub b1: Ball,
    pub b2: Ball,
    x_func: Option<Gamma>,
    y_func: Option<Gamma>,
}

impl Bone {
    pub fn new(b1: Ball, b2: Ball) -> Self {
        let x_func = None;
        let y_func = None;

        Bone {
            b1,
            b2,
            x_func,
            y_func,
        }
    }

    pub fn add_traceable(&self, tracer: &mut GroupTracer, world_view: &WorldView) {
        let proj1 = BallProjection::new(world_view, self.b1.clone());
        let proj2 = BallProjection::new(world_view, self.b2.clone());

        if self.x_func.is_none() && self.y_func.is_none() {
            let bone_tracer = BoneTracer::new(proj1, proj2);

            tracer.add(Tracer::BoneT(bone_tracer));

            return;
        }

        let c1 = self.b1.clone().color;
        let c2 = self.b2.clone().color;

        let v = self.b2.clone() - self.b1.clone();
        let length = v.length();
        let (vx, vy) = (v * 1.0 / length).cross_axes();

        let calc = |factor| {
            let color = c1.mix(c2, factor);

            let fx = match &self.x_func {
                Some(f) => f.call(factor),
                None => factor,
            };
            let fy = match &self.y_func {
                Some(f) => f.call(factor),
                None => factor,
            };

            let c = self.b1.clone()
                + v * factor
                + vx * ((fx - factor) * length)
                + vy * ((fy - factor) * length);
            let r = mix_floats(self.b1.radius, self.b2.radius, factor);

            BallProjection::new(
                world_view,
                Ball::new_v(String::from(""), c, r, color),
            )
        };

        let parts = 255;
        let prev = RefCell::new(proj1);
        let next = RefCell::new(calc(1.0 / parts as f64));

        for i in 1..parts {
            let current = next.borrow().clone();

            if i < parts {
                next.replace(calc((i + 1) as f64 / parts as f64));

                let seg1 = current.clone().base - prev.borrow().base.clone();
                let seg2 = next.borrow().base.clone() - current.clone().base;

                if seg1.scalar_product(seg2) / (seg1.length() * seg2.length()) > 0.999848 {
                    continue;
                }
            }

            tracer.add(Tracer::BoneT(BoneTracer::new(
                prev.borrow().clone(),
                current.clone(),
            )));

            prev.replace(current.clone());
        }
    }

    pub fn non_linear(b1: Ball, b2: Ball, x_func: Gamma, y_func: Gamma) -> Self {
        let x_func = Some(x_func);
        let y_func = Some(y_func);

        Bone {
            b1,
            b2,
            x_func,
            y_func,
        }
    }

    pub fn non_linear_y(b1: Ball, b2: Ball, y_func: Gamma) -> Self {
        let x_func = None;
        let y_func = Some(y_func);

        Bone {
            b1,
            b2,
            x_func,
            y_func,
        }
    }

    pub fn rotate_around(&self, other: &Vector, angle: f64, axis: Axis) {
        self.b1.rotate_around(other, angle, axis);
        self.b2.rotate_around(other, angle, axis);
    }
}

fn mix_floats(a: f64, b: f64, f: f64) -> f64 {
    a + f * (b - a)
}
