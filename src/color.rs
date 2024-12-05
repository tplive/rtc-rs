use std::ops;

use crate::util::{equal, RtcFl};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: RtcFl,
    pub green: RtcFl,
    pub blue: RtcFl,
}

impl Color {
    pub fn new(red: RtcFl, green: RtcFl, blue: RtcFl) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    
    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl ops::Mul<RtcFl> for Color {
    type Output = Self;

    fn mul(self, other: RtcFl) -> Self::Output {
        Self::new(self.red * other, self.green * other, self.blue * other)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, &other: &Color) -> bool {
        equal(self.red, other.red) && equal(self.green, other.green) && equal(self.blue, other.blue)
    }
}

pub fn color(r: RtcFl, g: RtcFl, b: RtcFl) -> Color {
    Color::new(r, g, b)
}
