use crate::Color;

use image::RgbaImage;

use std::convert::TryInto;

mod coloring_parameters;

pub use coloring_parameters::ColoringParameters;
pub use coloring_parameters::Gradient;

pub fn circle(
    image: &mut RgbaImage,
    cx: i32,
    cy: i32,
    r: i32,
    color: Color,
    coloring: ColoringParameters,
) {
    circle_impl(image, cx, cy, r, color, false, coloring);
}

pub fn circle_f(
    image: &mut RgbaImage,
    cx: f64,
    cy: f64,
    r: f64,
    color: Color,
    coloring: ColoringParameters,
) {
    circle(
        image,
        (cx + 0.5) as i32,
        (cy + 0.5) as i32,
        (r + 0.5) as i32,
        color,
        coloring,
    );
}

pub fn circle_shading_rgba(
    x: f64,
    y: f64,
    r: f64,
    color: Color,
    coloring: ColoringParameters,
) -> Color {
    if coloring.shading == 0.0 || y == 0.0 {
        return color;
    }

    let (sh, lighten) = match coloring.gradient {
        Gradient::Circly => {
            let sh1 = 1.0 - (1.0 - 1.0f64.min((y * y) / (r * r))).sqrt();
            let d = (x * x + y * y).sqrt() / r;
            let sh2 = y.abs() / r;

            let sh = (1.0 - d) * sh1 + d * sh2;

            (sh, 128.0)
        }
        Gradient::Distance => ((y / r).abs(), 255.0),
    };

    if y > 0.0 {
        color.darken((255.0 * sh * coloring.shading) as u8)
    } else {
        color.lighten((lighten * sh * coloring.shading) as u8)
    }
}

pub fn top_half_circle_f(
    image: &mut RgbaImage,
    cx: f64,
    cy: f64,
    r: f64,
    color: Color,
    coloring: ColoringParameters,
) {
    circle_impl(
        image,
        (cx + 0.5) as i32,
        (cy + 0.5) as i32,
        (r + 0.5) as i32,
        color,
        true,
        coloring,
    );
}

fn circle_impl(
    image: &mut RgbaImage,
    cx: i32,
    cy: i32,
    r: i32,
    color: Color,
    top_half: bool,
    coloring: ColoringParameters,
) {
    let size = image.height() as i32;

    if cx < -r || cy < -r || cx - r > size || cy - r > size {
        return;
    }

    let mut f = 1 - r;
    let mut dd_f_x = 1;
    let mut dd_f_y = -2 * r;
    let mut x = 0;
    let mut y = r;

    let mut fill = |mut left: i32, mut right: i32, mut y: i32| {
        left += cx;
        right += cx;

        y += cy;
        if left < 0 {
            left = 0;
        }
        if right >= size {
            right = size - 1;
        }

        for x in left..=right {
            let color = circle_shading_rgba(
                (x - cx) as f64,
                (y - cy) as f64,
                r as f64,
                color,
                coloring.clone(),
            );

            image.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), color.into());
        }
    };

    fill(-r, r, 0);

    while x < y {
        if f >= 0 {
            y -= 1;
            dd_f_y += 2;
            f += dd_f_y;
        }

        x += 1;
        dd_f_x += 2;
        f += dd_f_x;

        fill(-x, x, -y);
        fill(-y, y, -x);

        if top_half {
            return;
        };

        fill(-x, x, y);
        fill(-y, y, x);
    }
}
