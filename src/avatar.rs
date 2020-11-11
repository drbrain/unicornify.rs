extern crate image;

use anyhow::Context;
use anyhow::Result;

use crate::geometry::Axis;
use crate::geometry::Point;
use crate::geometry::Vector;
use crate::geometry::DEGREE;
use crate::render::WorldView;
use crate::scene::Background;
use crate::scene::Grass;
use crate::unicorn::Unicorn;
use crate::Color;
use crate::Data;
use crate::Random;

use image::RgbaImage;

pub struct Avatar {
    data: Data,
    scale_factor: f64,
    focal_length: f64,
    background: Background,
    unicorn: Unicorn,
}

impl Avatar {
    pub fn new(hash: String, zoom_out: bool) -> Result<Self> {
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

        let unicorn = Unicorn::new(&data);

        Ok(Avatar {
            data,
            scale_factor,
            focal_length,
            background,
            unicorn,
        })
    }

    pub fn draw(
        &self,
        size: u32,
        with_background: bool,
        zoom_out: bool,
        shading: bool,
        _grass: bool,
    ) -> RgbaImage {
        let fsize = size as f64;
        let factor = (self.scale_factor - 0.5).sqrt() / 2.5;

        let head = self.unicorn.head();
        let shoulder = self.unicorn.shoulder();
        let look_at_point = shoulder.clone() + ((head.clone() - shoulder) * factor);
        let camera_position = look_at_point + Vector::new(0.0, 0.0, -3.0 * self.focal_length);
        camera_position.rotate_around(*head.center.borrow(), -self.data.x_angle, Axis::X);
        camera_position.rotate_around(*head.center.borrow(), -self.data.y_angle, Axis::Y);

        let world_view = WorldView::new(camera_position, look_at_point, self.focal_length);

        let shift = Point::new(
            0.5 * fsize,
            factor * fsize / 3.0 + (1.0 - factor) * fsize / 2.0,
        );
        let scale = ((self.scale_factor - 0.5) / 2.5 * 2.0 + 0.5) * fsize / 140.0;

        let mut image_buffer = RgbaImage::new(size, size);

        if with_background {
            self.background.draw(&mut image_buffer, shading);
        }

        let tracer = self.unicorn.tracer(world_view);

        image_buffer
    }
}
