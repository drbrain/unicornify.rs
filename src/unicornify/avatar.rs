use anyhow::Context;
use anyhow::Result;

use crate::geometry::Vector;
use crate::geometry::DEGREE;
use crate::scene::Background;
use crate::scene::Grass;
use crate::unicornify::Unicorn;
use crate::Color;
use crate::Data;
use crate::Random;

pub struct Avatar {
    rand: Random,
    data: Data,
    size: usize,
}

impl Avatar {
    pub fn new(
        hash: String,
        size: usize,
        _with_background: bool,
        zoom_out: bool,
        _shading: bool,
        _glass: bool,
    ) -> Result<Self> {
        let mut rand = Random::new();

        rand.seed_hex_string(hash)
            .with_context(|| format!("Unable to use avatar hash"))?;

        let mut data = Data::new();
        let mut background = Background::new();
        let mut grass = Grass::new();

        data.rand1(&mut rand);
        background.rand1(&mut rand);

        let scale_factor = 0.5 + rand.rand().powi(2) * 2.5;

        let scale_factor = if zoom_out { 0.5 } else { scale_factor };

        let sign = rand.choice(2) * 2 - 1;
        let abs = rand.rand_i32(10, 75);
        data.y_angle = 90.0 + sign as f64 * abs as f64 * DEGREE;
        data.x_angle = rand.rand_i32(-20, 20) as f64 * DEGREE;

        data.rand2(&mut rand);
        background.rand2(&mut rand);
        data.rand3(&mut rand);
        grass.rand(&mut rand);

        let grass_slope = 2.0 + 4.0 * (20.0 - data.x_angle / DEGREE) / 40.0;
        let grass_scale = 1.0 + (scale_factor - 0.5) / 2.5;
        grass.blade_height_near = (0.02 + 0.02 * rand.rand()) * grass_scale;
        grass.blade_height_far = grass.blade_height_near / grass_slope;

        let focal_length = 250.0 + rand.rand() * 250.0;

        data.rand4(&mut rand);

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

        if (data.y_angle - 90.0 * DEGREE) * data.neck_tilt > 0.0 {
            data.neck_tilt = -data.neck_tilt;
            data.face_tilt = -data.face_tilt;
        }

        Ok(Avatar { rand, data, size })
    }
}
