use crate::{colors::Color, util::RtcFl};

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub data: Vec<RtcFl>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {

        let bits = 3;
        let data = vec![0.0; (width * height * bits) as usize];

        Self {
            width: width,
            height: height,
            data:data,
        }
    }

    pub fn write_pixel(&self, x: u32, y: u32, color: Color) {
    }

    pub fn pixel_at(&self, x, y) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

}
