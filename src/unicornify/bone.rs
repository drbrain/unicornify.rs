use crate::unicornify::Ball;
use crate::Gamma;

const DEFAULT_SHADING: f64 = 0.25;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
}
