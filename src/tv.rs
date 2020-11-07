use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub struct TV {
    pub t: f64,
    pub v: f64,
}

impl TV {
    pub fn new(t: f64, v: f64) -> Self {
        TV { t, v }
    }
}

impl Ord for TV {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.t <= other.t, self.t >= other.t) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => panic!("Incomparable floats {:?} and {:?}", self.t, other.t),
        }
    }
}

impl PartialOrd for TV {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TV {}

impl PartialEq for TV {
    fn eq(&self, other: &Self) -> bool {
        self.t.eq(&other.t)
    }
}
