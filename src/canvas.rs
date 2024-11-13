use std::ops::Index;

use crate::{colors::Color, util::RtcFl};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    data: Vec<RtcFl>,
    bits: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {

        let bits: usize = 3;
        let mut data: Vec<RtcFl> = vec![0.0; width * height * bits];

        Self {
            width,
            height,
            data,
            bits,
        }
    }

    pub fn bits(&self) -> usize { self.bits}

    pub fn write_pixel(&self, x: usize, y: usize, color: Color) {
        let index: usize = (y * self.width + x) * self.bits;
        &mut self.data.index(index) = color.red;
        &mut self.data[index+1] = color.green;
        &mut self.data[index+2] = color.blue;
    }

    pub fn pixel_at(&self, x, y) -> Color {
        let index = y * self.width + x;
        let r = self.data[index];
        let g = self.data[index+1];
        let b = self.data[index+2];
        Color::new(r, g, b)
    }

}
