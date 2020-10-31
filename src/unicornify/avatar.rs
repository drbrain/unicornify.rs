use anyhow::Context;
use anyhow::Result;

use crate::Color;
use crate::Random;

use crate::unicornify::Background;
use crate::unicornify::Grass;
use crate::unicornify::Unicorn;
use crate::unicornify::Vector;
use crate::unicornify::DEGREE;

pub struct Avatar {
    rand: Random,
    unicorn: Unicorn,
}

impl Avatar {
    pub fn new(
        hash: String,
        _size: usize,
        _with_background: bool,
        zoom_out: bool,
        _shading: bool,
        _glass: bool,
    ) -> Result<Self> {
        let mut rand = Random::new();

        rand.seed_hex_string(hash)
            .with_context(|| format!("Unable to use avatar hash"))?;

        let mut unicorn = Unicorn::new();
        let mut background = Background::new();
        let mut grass = Grass::new();

        unicorn.rand1(&mut rand);
        background.rand1(&mut rand);

        let scale_factor = 0.5 + rand.rand().powi(2) * 2.5;

        let scale_factor = if zoom_out { 0.5 } else { scale_factor };

        let sign = rand.choice(2) * 2 - 1;
        let abs = rand.rand_i32(10, 75);
        let y_angle = 90.0 + sign as f64 * abs as f64 * DEGREE;
        let x_angle = rand.rand_i32(-20, 20) as f64 * DEGREE;

        unicorn.rand2(&mut rand);
        background.rand2(&mut rand);
        unicorn.rand3(&mut rand);
        grass.rand(&mut rand);

        let grass_slope = 2.0 + 4.0 * (20.0 - x_angle / DEGREE) / 40.0;
        let grass_scale = 1.0 + (scale_factor - 0.5) / 2.5;
        grass.blade_height_near = (0.02 + 0.02 * rand.rand()) * grass_scale;
        grass.blade_height_far = grass.blade_height_near / grass_slope;

        let focal_length = 250.0 + rand.rand() * 250.0;

        unicorn.rand4(&mut rand);

        let light_direction = Vector::new(rand.rand() * 16.0 - 8.0, 10.0, rand.rand() * 3.0);
        let light_direction = Vector::new(light_direction.z, light_direction.y, -light_direction.x);

        // end randomization

        grass.horizon = background.horizon;
        grass.color1 = Color::hsl(
            background.land_hue,
            background.land_sat,
            background.land_light,
        );
        grass.color2 = Color::hsl(
            background.land_hue,
            background.land_sat,
            background.land_light / 2,
        );

        if (y_angle - 90.0 * DEGREE) * unicorn.neck_tilt > 0.0 {
            unicorn.neck_tilt = -unicorn.neck_tilt;
            unicorn.face_tilt = -unicorn.face_tilt;
        }

        Ok(Avatar { rand, unicorn })
    }
}
