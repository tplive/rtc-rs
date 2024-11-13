use std::ops;

use crate::util::equal;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {
            red: red,
            green: green,
            blue: blue,
        }
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

impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self::new(
            self.red * other,
            self.green * other,
            self.blue * other,
        )
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

pub fn color(r: f32, g: f32, b: f32) -> Color {
    Color::new(r, g, b)
}


