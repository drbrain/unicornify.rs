use crate::TV;

pub struct Sorter {
    l: Vec<TV>,
}

impl Sorter {
    pub fn new(mut tvs: Vec<TV>) -> Self {
        tvs.sort();
        let first = tvs.first().unwrap();
        let last = tvs.last().unwrap();

        let mut l = Vec::with_capacity(tvs.len() + 2);
        l.push(TV::new(last.t - 1.0, last.v));
        l.push(TV::new(first.t + 1.0, first.v));
        l.copy_from_slice(&tvs[..]);

        Sorter { l }
    }

    pub fn interpolate(&self, t: f64) -> f64 {
        let t = t.fract();

        let mut t1 = -2.0;
        let mut t2 = 2.0;
        let mut v1 = 0.0;
        let mut v2 = 0.0;

        for tv in self.l.iter() {
            if tv.t <= t && tv.t > t1 {
                t1 = tv.t;
                v1 = tv.v;
            }

            if tv.t >= t && tv.t < t2 {
                t2 = tv.t;
                v2 = tv.v;
            }
        }

        if t1 == t2 {
            v1
        } else {
            mix_floats(v1, v2, (t - t1) / (t2 - t1))
        }
    }
}

fn mix_floats(f1: f64, f2: f64, f: f64) -> f64 {
    f1 + f * (f2 - f1)
}
