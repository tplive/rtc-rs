use crate::{color::Color, tuples::Tuple};
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl Light {
    pub fn point(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
