use crate::drawing::ColoringParameters;
use crate::drawing::*;
use crate::geometry::Point;
use crate::Color;
use crate::Random;

use image::Rgba;
use image::RgbaImage;

use std::convert::TryInto;

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

    pub fn draw(&self, image: &mut RgbaImage, shaded: bool, quadrant: Option<u8>) {
        let height = match quadrant {
            None => image.height(),
            Some(_) => image.height() * 2,
        };

        let fsize = (height - 1) as f64;
        let horizon = (height as f64 * self.horizon) as u32;

        self.draw_sky(image, horizon, fsize, quadrant);
        self.draw_land(image, horizon, fsize, quadrant);
        self.draw_rainbow(image, horizon, fsize);
        for i in 0..self.cloud_positions.len() {
            self.draw_cloud(image, i, shaded, fsize);
        }
    }

    fn draw_cloud(&self, image: &mut RgbaImage, i: usize, shaded: bool, fsize: f64) {
        let image_size: u32 = image.width();
        let position = &self.cloud_positions[i];
        let size = &self.cloud_sizes[i];
        let color = Color::hsl(self.sky_hue, self.sky_sat, self.cloud_lightnesses[i]);

        let shading = if shaded { 0.25 } else { 0.0 };

        let cp = ColoringParameters::new(shading);

        let x = position.x * fsize;
        let y = position.y * fsize;
        let size1 = size.x * fsize;
        let size2 = size.x * size.y * fsize;

        circle_f(image, x - 2.0 * size1, y - size1, size1, color, cp.clone());
        circle_f(image, x + 2.0 * size1, y - size1, size1, color, cp.clone());
        top_half_circle_f(image, x, y - size1, size2, color, cp.clone());

        let xi = (x + 0.5) as i32;
        let yi = (y + 0.5) as i32;

        let size1i = (size1 + 0.5) as i32;
        let right = xi + 2 * size1i;

        for py in yi - size1i - 1..=yi {
            if py < 0 {
                continue;
            }

            for px in xi - 2 * size1i..=right {
                if px < 0 {
                    continue;
                }

                let image_x: u32 = px.try_into().unwrap();
                let image_y: u32 = py.try_into().unwrap();

                if image_x >= image_size || image_y >= image_size {
                    continue;
                }

                if shaded {
                    let dy = (py - (yi - size1i - 1)) as f64;
                    let color = circle_shading_rgba(0.0, dy, size1, color, cp.clone());

                    image.put_pixel(image_x, image_y, color.into());
                } else {
                    image.put_pixel(image_x, image_y, color.into());
                }
            }
        }
    }

    fn draw_land(&self, image: &mut RgbaImage, horizon: u32, fsize: f64, quadrant: Option<u8>) {
        let land_a = Color::hsl(self.land_hue, self.land_sat, self.land_light);
        let land_b = Color::hsl(self.land_hue, self.land_sat, self.land_light / 2);
        let (offset_x, offset_y) = offset(image.width(), quadrant);
        let edge = horizon - offset_y;

        for x in 0..image.height() {
            let color = land_a.mix(land_b, (x + offset_x) as f64 / fsize);

            for y in edge..image.width() {
                image.put_pixel(x, y, color.into());
            }
        }
    }

    fn draw_rainbow(&self, image: &mut RgbaImage, horizon: u32, fsize: f64) {
        let band_width = self.rainbow_band_width * fsize;
        let rainbow_center = fsize * (self.rainbow_foot + self.rainbow_dir * self.rainbow_height);
        let outer_radius = self.rainbow_height * fsize + 0.5;

        // TODO these can probably be u32 as self.rainbow_* all seem to be positive?
        let r = (outer_radius + 0.5) as i32;
        let cx = (rainbow_center + 0.5) as i32;
        let cy = horizon as i32;

        let size = image.height();
        let left = between(cx - r, 0, size - 1);
        let right = between(cx + r, 0, size - 1);
        let top = between(cx - r, 0, size - 1);
        let bottom = between(cy, 0, size - 1);
        let inner_radius_squared = (r as f64 - 7.0 * band_width) as u32;

        let band_colors = band_colors();

        for x in left..=right {
            let dx: i32 = x as i32 - cx;

            for y in top..=bottom {
                let dy: i32 = y as i32 - cy;
                let d_squared: u32 = (dx * dx + dy * dy).try_into().unwrap();

                if d_squared < inner_radius_squared {
                    continue;
                }

                let d = (d_squared as f64).sqrt();

                let band = (r as f64 - d) / band_width;

                if band >= 7.0 || band < 0.0 {
                    continue;
                }

                let band = band as u32;

                image.put_pixel(x, y, band_colors[band as usize]);
            }
        }
    }

    fn draw_sky(&self, image: &mut RgbaImage, horizon: u32, fsize: f64, quadrant: Option<u8>) {
        let sky_a = Color::hsl(self.sky_hue, self.sky_sat, 60);
        let sky_b = Color::hsl(self.sky_hue, self.sky_sat, 10);
        let (_, offset_y) = offset(image.width(), quadrant);
        let edge = offset_y + horizon;

        for (y, row) in image.enumerate_rows_mut() {
            if y >= edge {
                break;
            };

            let color = sky_a.mix(sky_b, (y + offset_y) as f64 / fsize);

            for (_x, _y, pixel) in row {
                *pixel = color.into();
            }
        }
    }

    pub fn rand1(&mut self, rand: &mut Random) {
        self.sky_hue = rand.rand_i32(0, 359);
        self.sky_sat = rand.rand_i32(30, 70);
        self.land_hue = rand.rand_i32(0, 359);
        self.land_sat = rand.rand_i32(20, 60);
        self.horizon = 0.5 + rand.rand() * 0.2;
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

fn band_colors() -> Vec<Rgba<u8>> {
    let mut band_colors: Vec<Rgba<u8>> = Vec::with_capacity(7);

    for i in 0..band_colors.capacity() {
        let rgb = Color::hsl(i as i32 * 45, 100, 50);

        band_colors.push(rgb.into());
    }

    band_colors
}

fn between(v: i32, min: u32, max: u32) -> u32 {
    let (min, max) = if min > max { (max, min) } else { (min, max) };

    let v: u32 = v.try_into().unwrap_or(0);

    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

fn offset(size: u32, quadrant: Option<u8>) -> (u32, u32) {
    match quadrant {
        None => (0, 0),
        Some(1) => (0, 0),
        Some(2) => (size, 0),
        Some(3) => (0, size),
        Some(4) => (size, size),
        Some(q) => panic!("Invalid quadrant {}", q),
    }
}
