use crate::Color;

use crate::unicornify::Ball;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Steak {
    pub b1: Ball,
    pub b2: Ball,
    pub b3: Ball,
    fourth_color: Color,
}

impl Steak {
    pub fn new(b1: Ball, b2: Ball, b3: Ball) -> Self {
        let fourth_color = Color::new(128, 128, 128);

        Steak {
            b1,
            b2,
            b3,
            fourth_color,
        }
    }
}
