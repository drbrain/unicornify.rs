use crate::geometry::Vector;
use crate::render::prune_bounds;
use crate::render::BallProjection;
use crate::render::Bounds;
use crate::render::RenderingParameters;
use crate::render::TraceResult;
use crate::render::Tracer;

#[derive(Clone, Debug, PartialEq)]
pub struct BoneTracer {
    w1: f64,
    w2: f64,
    w3: f64,
    a1: f64,
    a2: f64,
    a3: f64,
    ra: f64,
    dr: f64,
    c2: f64,
    c4: f64,
    c6: f64,
    c8: f64,
    c9: f64,
    c11: f64,
    c14: f64,
    c2i: f64,
    b1: BallProjection,
    b2: BallProjection,
    pub bounds: Bounds,
}

impl BoneTracer {
    pub fn new(b1: BallProjection, b2: BallProjection) -> Self {
        let cx1 = b1.sphere.center_cs.x;
        let cy1 = b1.sphere.center_cs.y;
        let cz1 = b1.sphere.center_cs.z;
        let r1 = b1.base.radius;

        let cx2 = b2.sphere.center_cs.x;
        let cy2 = b2.sphere.center_cs.y;
        let cz2 = b2.sphere.center_cs.z;
        let r2 = b2.base.radius;

        let bounds = Bounds::for_balls(vec![b1.clone(), b2.clone()]);

        let w1 = cx2 - cx1;
        let w2 = cy2 - cy1;
        let w3 = cz2 - cz1;

        let a1 = cx1;
        let a2 = cy1;
        let a3 = cz1;

        let ra = r1;
        let dr = r2 - r1;

        let c2 = -(dr * dr) + w1 * w1 + w2 * w2 + w3 * w3;
        let c2i = if c2 != 0.0 { 1.0 / c2 } else { 0.0 };

        let c4 = -2.0 * ra * dr + 2.0 * (a1 * w1 + a2 * w2 + a3 * w3);
        let c6 = -(ra * ra) + a1 * a1 + a2 * a2 + a3 * a3;
        let c8 = c4 / c2;
        let c9 = 1.0 / c2;
        let c11 = c6 / c2;
        let c14 = c8 * c8 / 4.0 - c11;

        BoneTracer {
            w1,
            w2,
            w3,
            a1,
            a2,
            a3,
            ra,
            dr,
            c2,
            c4,
            c6,
            c8,
            c9,
            c11,
            c14,
            c2i,
            b1,
            b2,
            bounds,
        }
    }

    pub fn prune(&self, rendering_parameters: &RenderingParameters) -> Option<Tracer> {
        prune_bounds(Tracer::BoneT(self.clone()), rendering_parameters)
    }

    pub fn trace(&self, _x: f64, _y: f64, ray: Vector) -> TraceResult {
        let v1 = ray.x;
        let v2 = ray.y;
        let v3 = ray.z;

        let c3 = -2.0 * (v1 * self.w1 + v2 * self.w2 + v3 * self.w3);
        let c5 = -2.0 * (v1 * self.a1 + v2 * self.a2 + v3 * self.a3);

        let mut z: f64 = 0.0;
        let mut f: f64 = 0.0;

        if self.c2 == 0.0 {
            f = if self.dr > 0.0 { 1.0 } else { 0.0 };
        } else {
            let c7 = c3 * self.c2i;
            let c10 = c5 * self.c2i;
            let c12i = 1.0 / (c7 * c7 / 4.0 - self.c9);
            let c13 = c7 * self.c8 / 2.0 - c10;

            let pz = c13 * c12i;
            let qz = self.c14 * c12i;
            let discz = pz * pz / 4.0 - qz;

            if discz < 0.0 {
                return None;
            }

            let rdiscz = discz.sqrt();
            let mut z1 = -pz / 2.0 + rdiscz;
            let mut z2 = -pz / 2.0 - rdiscz;
            let mut f1 = -(c3 * z1 + self.c4) / (2.0 * self.c2);
            let mut f2 = -(c3 * z2 + self.c4) / (2.0 * self.c2);

            let g1 = self.ra + f1 * self.dr >= 0.0;
            let g2 = self.ra + f2 * self.dr >= 0.0;

            if !g1 {
                z1 = z2;
                f1 = f2;

                f2 = if self.dr > 0.0 { 1.0 } else { 0.0 };
            }

            if !g2 {
                z2 = z1;
                f2 = f1;

                f1 = if self.dr > 0.0 { 1.0 } else { 0.0 };
            }

            // backside
            z = z2;
            f = f2;
        }

        if f <= 0.0 || f >= 1.0 {
            f = 1.0f64.min(0.0f64.max(f));
            let mut pz = c3 * f + c5;
            let mut qz = self.c2 * f * f + self.c4 * f + self.c6;
            let mut discz = pz * pz / 4.0 - qz;

            if discz < 0.0 {
                f = 1.0 - f;
                pz = c3 * f + c5;
                qz = self.c2 * f * f + self.c4 * f + self.c6;
                discz = pz * pz / 4.0 - qz;

                if discz < 0.0 {
                    return None;
                }

                // backside
                z = -pz / 2.0 - discz.sqrt();
            }
        }

        let m1 = self.a1 + f * self.w1;
        let m2 = self.a2 + f * self.w2;
        let m3 = self.a3 + f * self.w3;
        let m = Vector::new(m1, m2, m3);

        let p = ray * z;
        let dir = p - m;

        Some((z, dir, self.b1.base.color.mix(self.b2.base.color, f)))
    }
}
