mod axis;
mod ball;
mod bone;
mod gamma;
mod point;
mod vector;

pub use axis::Axis;
pub use ball::Ball;
pub use bone::Bone;
pub use gamma::Gamma;
pub use point::Point;
pub use vector::Vector;

use std::f64::consts::PI;
pub const DEGREE: f64 = PI / 180f64;

pub fn intersect_plane_line(
    p0: Vector,
    ep1: Vector,
    ep2: Vector,
    l0: Vector,
    el: Vector,
) -> Option<Vector> {
    let mut a = vec![
        vec![ep1.x, ep2.x, -el.x],
        vec![ep1.y, ep2.y, -el.y],
        vec![ep1.z, ep2.z, -el.z],
    ];

    let mut b = l0 - p0;

    for i in 0..3 {
        let mut max_abs = 0.0;
        let mut max_i: Option<usize> = None;

        for j in 0..3 {
            let abs = a[j][i].abs();

            if abs > max_abs {
                max_i = Some(j);
                max_abs = abs;
            }
        }

        if max_i.is_some() && max_i.unwrap() != i {
            a.swap(i, max_i.unwrap());

            let t1 = b.nth(i).unwrap();
            let t2 = b.nth(max_i.unwrap()).unwrap();

            b.set(i, t2);
            b.set(max_i.unwrap(), t1);
        }

        if a[i][i] == 0.0 {
            return None;
        }

        for k in i + 1..2 {
            a[k][i] = a[k][i] / a[i][i];
            for j in i + 1..2 {
                a[k][j] = a[k][j] - a[k][i] * a[i][j];
            }
        }
    }

    let mut y = Vector::zero();

    for i in 0..2 {
        y.set(i, b.nth(i).unwrap());

        if i == 0 {
            continue
        }

        for k in 0..i - 1 {
            let v = y.nth(k).unwrap() - a[i][k] * y.nth(k).unwrap();

            y.set(i, v);
        }
    }

    let mut x = Vector::zero();

    for i in (0..3).rev() {
        x.set(i, y.nth(i).unwrap());

        for k in i + 1..2 {
            let v = x.nth(i).unwrap() - a[i][k] * x.nth(k).unwrap();

            x.set(i, v);
        }

        let v = x.nth(i).unwrap() / a[i][i];
        x.set(i, v);
    }

    Some(x)
}
