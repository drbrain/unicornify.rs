use crate::render::BallProjection;
use crate::render::Bounds;

#[derive(Clone, Debug)]
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
        let w2 = cy2 - cy2;
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
}
