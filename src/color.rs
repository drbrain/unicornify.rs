pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, b: u8, g: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn black() -> Self {
        Color::new(0, 0, 0, 255)
    }
}
