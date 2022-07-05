/// Color.rs

pub struct Color {
    r: i16,
    g: i16,
    b: i16,
    a: i16
}

impl Color {
    pub fn new(r: i16, g: i16, b: i16) -> Color {
        Color {
            r: r, g: g, b: b, a: 255
        }
    }
}