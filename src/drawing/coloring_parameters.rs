#[derive(Clone, Debug)]
pub enum Gradient {
    Circly,
    Distance,
}

#[derive(Clone, Debug)]
pub struct ColoringParameters {
    pub shading: f64,
    pub gradient: Gradient,
}

impl ColoringParameters {
    pub fn new(shading: f64) -> Self {
        let gradient = Gradient::Circly;

        ColoringParameters { shading, gradient }
    }
}
