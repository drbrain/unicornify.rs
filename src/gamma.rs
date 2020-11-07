use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, Debug, PartialEq)]
pub struct Gamma {
    gamma: f64,
    t: f64,
}

impl Gamma {
    pub fn new(gamma: f64, t: f64) -> Self {
        Gamma { gamma, t }
    }

    pub fn call(&self, x: f64) -> f64 {
        self.t * x.powf(self.gamma) + (1.0 - self.t) * x
    }
}

impl Eq for Gamma {}

impl Hash for Gamma {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let gamma_val = format!("{:.10e}", self.gamma);
        gamma_val.hash(state);

        let t_val = format!("{:.10e}", self.t);
        t_val.hash(state);
    }
}
