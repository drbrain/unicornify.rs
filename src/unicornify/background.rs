use crate::Point;
use crate::Random;

pub struct Background {
    pub sky_hue: i32,
    pub sky_sat: i32,
    pub land_hue: i32,
    pub land_sat: i32,
    pub horizon: f64,
    pub rainbow_foot: f64,
    pub rainbow_dir: f64, // +1 or -1
    pub rainbow_height: f64,
    pub rainbow_band_width: f64,
    pub cloud_positions: Vec<Point>,
    pub cloud_sizes: Vec<Point>, // not actually any kind of point
    pub cloud_lightnesses: Vec<i32>,
    pub land_light: i32,
}

impl Background {
    pub fn new() -> Self {
        Background {
            sky_hue: 0,
            sky_sat: 0,
            land_hue: 0,
            land_sat: 0,
            horizon: 0.0,
            rainbow_foot: 0.0,
            rainbow_dir: 0.0,
            rainbow_height: 0.0,
            rainbow_band_width: 0.0,
            cloud_positions: vec![],
            cloud_sizes: vec![],
            cloud_lightnesses: vec![],
            land_light: 0,
        }
    }

    pub fn rand1(&mut self, rand: &mut Random) {
        self.sky_hue = rand.rand_i32(0, 359);
        self.sky_sat = rand.rand_i32(30, 70);
        self.land_hue = rand.rand_i32(0, 359);
        self.land_sat = rand.rand_i32(20, 60);
        self.horizon = 0.5 + rand.rand() * 2.0;
        self.rainbow_foot = 0.2 + rand.rand() * 0.6;
        self.rainbow_dir = (rand.choice(2) * 2 - 1) as f64;
        self.rainbow_height = 0.5 + rand.rand() * 1.5;
        self.rainbow_band_width = 0.01 + rand.rand() * 0.02;
        self.land_light = rand.rand_i32(20, 50);
    }

    pub fn rand2(&mut self, rand: &mut Random) {
        let cloud_count: usize = rand.rand_i32(1, 3) as usize;

        self.cloud_positions.reserve(cloud_count);
        self.cloud_sizes.reserve(cloud_count);
        self.cloud_lightnesses.reserve(cloud_count);

        for _ in 0..cloud_count {
            let cloud_position = Point::new(rand.rand(), (0.3 + rand.rand() * 0.6) * self.horizon);
            self.cloud_positions.push(cloud_position);
        }

        for _ in 0..cloud_count {
            let cloud_size = Point::new(rand.rand() * 0.04 + 0.02, rand.rand() * 0.7 + 1.3);
            self.cloud_sizes.push(cloud_size);
        }

        for _ in 0..cloud_count {
            self.cloud_lightnesses.push(rand.rand_i32(75, 90));
        }
    }
}
