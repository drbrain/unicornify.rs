use crate::render::Bounds;

#[derive(Clone, Debug)]
pub struct RenderingParameters {
    pub pixel_size: f64,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

impl RenderingParameters {
    pub fn new(pixel_size: f64, bounds: Bounds) -> Self {
        let x_min = (bounds.x_min - 1.0).floor();
        let x_max = (bounds.x_max + 1.0).ceil();
        let y_min = (bounds.y_min - 1.0).floor();
        let y_max = (bounds.y_max + 1.0).ceil();

        RenderingParameters {
            pixel_size,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn any_infinite(&self) -> bool {
        self.x_min.is_infinite()
            || self.x_max.is_infinite()
            || self.y_min.is_infinite()
            || self.y_max.is_infinite()
    }

    pub fn contains(&self, bounds: &Bounds) -> bool {
        bounds.x_max >= self.x_min
            && bounds.x_min <= self.x_max
            && bounds.y_max >= self.y_min
            && bounds.y_min <= self.y_max
            && bounds.z_max > 0.0
    }

    pub fn scale(&self, scale: f64) -> Self {
        let pixel_size = self.pixel_size / scale;
        let x_min = self.x_min / scale;
        let x_max = self.x_max / scale;
        let y_min = self.y_min / scale;
        let y_max = self.y_max / scale;

        RenderingParameters {
            pixel_size,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn translated(&self, dx: f64, dy: f64) -> Self {
        let pixel_size = self.pixel_size;
        let x_min = self.x_min - dx;
        let x_max = self.x_max - dx;
        let y_min = self.y_min - dy;
        let y_max = self.y_max - dy;

        RenderingParameters {
            pixel_size,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}
