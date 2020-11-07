#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, b: u8, g: u8) -> Self {
        Color { r, g, b }
    }

    pub fn hsl(hue: i32, saturation: i32, lightness: i32) -> Self {
        if saturation == 0 {
            return Color {
                r: 255,
                g: 255,
                b: 255,
            };
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
        }
    }

    pub fn black() -> Self {
        Color::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Color::new(255, 255, 255)
    }
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
