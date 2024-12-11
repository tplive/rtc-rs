use crate::{color::Color, util::RtcFl};
pub struct Material {
    pub color: Color,
    pub ambient: RtcFl,
    pub diffuse: RtcFl,
    pub specular: RtcFl,
    pub shininess: RtcFl,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: RtcFl,
        diffuse: RtcFl,
        specular: RtcFl,
        shininess: RtcFl,
    ) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}
