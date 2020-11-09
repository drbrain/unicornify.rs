use image::Rgba;
use std::convert::Into;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        let a = 255;

        Color { r, g, b, a }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn hsl(hue: i32, saturation: i32, lightness: i32) -> Self {
        if saturation == 0 {
            return Color::rgb(255, 255, 255);
        }

        let h = (hue as f64) / 360.0;
        let s = (saturation as f64) / 100.0;
        let l = (lightness as f64) / 100.0;

        let m2 = if l <= 0.5 {
            l * (1.0 + s)
        } else {
            l + s - (l * s)
        };
        let m1 = 2.0 * l - m2;
        let rf = v(m1, m2, h + 1.0 / 3.0);
        let gf = v(m1, m2, h);
        let bf = v(m1, m2, h - 1.0 / 3.0);

        Color {
            r: to_u8(255.0 * rf),
            g: to_u8(255.0 * gf),
            b: to_u8(255.0 * bf),
            a: 255,
        }
    }

    pub fn black() -> Self {
        Color::rgb(0, 0, 0)
    }

    pub fn darken(&self, d: u8) -> Self {
        let r = self.r - std::cmp::min(d, self.r);
        let g = self.g - std::cmp::min(d, self.g);
        let b = self.b - std::cmp::min(d, self.b);
        let a = self.a;

        Color::rgba(r, g, b, a)
    }

    pub fn lighten(&self, d: u8) -> Self {
        let r = self.r + std::cmp::min(d, 255 - self.r);
        let g = self.g + std::cmp::min(d, 255 - self.g);
        let b = self.b + std::cmp::min(d, 255 - self.b);
        let a = self.a;

        Color::rgba(r, g, b, a)
    }

    pub fn mix(&self, other: Color, f: f64) -> Color {
        let r = mix_u8(self.r, other.r, f);
        let g = mix_u8(self.g, other.g, f);
        let b = mix_u8(self.b, other.b, f);

        Color::rgb(r, g, b)
    }

    pub fn white() -> Self {
        Color::rgb(255, 255, 255)
    }
}

impl Into<Rgba<u8>> for Color {
    fn into(self) -> Rgba<u8> {
        Rgba([self.r, self.g, self.b, self.a])
    }
}

fn mix_u8(a: u8, b: u8, f: f64) -> u8 {
    a + to_u8(f * (b as f64 - a as f64) + 0.5)
}

fn to_u8(float: f64) -> u8 {
    (255.0 * float).round() as u8
}

fn v(m1: f64, m2: f64, hue: f64) -> f64 {
    let hue = hue.fract();
    let hue = (hue + 1.0).fract();
    if hue < 1.0 / 6.0 {
        return m1 + (m2 - m1) * hue * 6.0;
    }
    if hue < 0.5 {
        return m2;
    }
    if hue < 2.0 / 3.0 {
        m1 + (m2 - m1) * (2.0 / 3.0 - hue) * 6.0
    } else {
        m1
    }
}
