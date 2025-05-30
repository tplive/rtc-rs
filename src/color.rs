use std::ops;

use crate::util::{equal, RtcFl};
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: RtcFl,
    pub green: RtcFl,
    pub blue: RtcFl,
}

impl Color {
    pub fn new(red: RtcFl, green: RtcFl, blue: RtcFl) -> Self {
        Self { red, green, blue }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        Self::new(
            rng.random::<RtcFl>(),
            rng.random::<RtcFl>(),
            rng.random::<RtcFl>(),
        )
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

#[cfg(test)]
mod tests {
    use crate::color::color;

    #[test]
    fn colors_are_rgb_tuples() {
        let c = color(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);

        let expected_result = color(1.6, 0.7, 1.0);
        let actual_result = c1 + c2;

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);

        let expected_result = color(0.2, 0.5, 0.5);
        let actual_result = c1 - c2;

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c1 = color(0.2, 0.3, 0.4);
        let multiplier = 2.0;

        let expected_result = color(0.4, 0.6, 0.8);
        let actual_result = c1 * multiplier;

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn multiply_two_colors() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);

        let expected_result = color(0.9, 0.2, 0.04);
        let actual_result = c1 * c2;

        assert_eq!(expected_result, actual_result);
    }
}
