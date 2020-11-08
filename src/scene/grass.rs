use crate::Color;
use crate::Random;

pub struct Grass {
    pub seed: u32,
    pub row_seed_add: u32,
    pub horizon: f64,
    pub blade_height_far: f64, // 0-1-based, relative to image width/height
    pub blade_height_near: f64,
    pub wind: f64,
    pub color1: Color,
    pub color2: Color,
    pub min_bottom_y: f64, // pixel
}

impl Grass {
    pub fn new() -> Self {
        Grass {
            seed: 0,
            row_seed_add: 0,
            horizon: 0.0,
            blade_height_far: 0.0,
            blade_height_near: 0.0,
            wind: 0.0,
            color1: Color::black(),
            color2: Color::black(),
            min_bottom_y: 0.0,
        }
    }

    pub fn rand(&mut self, rand: &mut Random) {
        let r = rand.rand_bits(64);
        self.seed = r[0];
        self.row_seed_add = r[1];
        self.wind = 1.6 * rand.rand() - 0.8;
    }
}
