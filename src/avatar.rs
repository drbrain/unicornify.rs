extern crate image;

use anyhow::Context;
use anyhow::Result;

use crate::geometry::Axis;
use crate::geometry::Point;
use crate::geometry::Vector;
use crate::geometry::DEGREE;
use crate::render::QuadrantTracer;
use crate::render::ScalingTracer;
use crate::render::Tracer;
use crate::render::TranslatingTracer;
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
        data.y_angle = (90 + sign * abs) as f64 * DEGREE;
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
        let _light_direction = Vector::new(light_direction.z, light_direction.y, -light_direction.x);

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
            data.neck_tilt *= -1.0;
            data.face_tilt *= -1.0;
        }

        let unicorn = Unicorn::new(&mut data);

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
        quadrant: Option<u8>,
        with_background: bool,
        _zoom_out: bool,
        shading: bool,
        grass: bool,
        parallelize: bool,
    ) -> RgbaImage {
        let fsize = size as f64;
        let factor = ((self.scale_factor - 0.5) / 2.5).sqrt();

        let head = self.unicorn.head();
        let shoulder = self.unicorn.shoulder();
        let look_at = shoulder.clone() + ((head.clone() - shoulder) * factor);

        let pivot = &head.center.borrow();
        let camera = look_at + Vector::new(0.0, 0.0, -3.0 * self.focal_length);
        let camera = camera.rotate_around(pivot, -self.data.x_angle, Axis::X);
        let camera = camera.rotate_around(pivot, -self.data.y_angle, Axis::Y);

        let world_view = WorldView::new(camera, look_at, self.focal_length);

        let shift = Point::new(
            0.5 * fsize,
            factor * fsize / 3.0 + (1.0 - factor) * fsize / 2.0,
        );

        let scale = ((self.scale_factor - 0.5) / 2.5 * 2.0 + 0.5) * fsize / 140.0;

        let image_size = match quadrant {
            Some(_) => size / 2,
            None => size,
        };

        let mut image_buffer = RgbaImage::new(image_size, image_size);

        if with_background {
            self.background.draw(&mut image_buffer, shading, quadrant);
        }

        let tracer = self.unicorn.tracer(&world_view);

        if shading {
            todo!("Implement shadow casting");
        }

        let scaling = ScalingTracer::new(&world_view, Tracer::GroupT(tracer), scale);
        let translating = TranslatingTracer::new(&world_view, Tracer::ScalingT(scaling), shift);

        if grass {
            todo!("Implement grass");
        }

        let tracer = if parallelize {
            todo!("Implement parallel tracing");
        } else {
            Tracer::TranslatingT(translating)
        };

        let mut tracer = match quadrant {
            None => tracer,
            Some(q) => Tracer::QuadrantT(QuadrantTracer::new(&world_view, tracer, image_size, q)),
        };

        tracer.draw(world_view, &mut image_buffer);

        image_buffer
    }
}
