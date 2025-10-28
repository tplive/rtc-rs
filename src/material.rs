use crate::{color::Color, util::RtcFl};

#[derive(Debug, Copy, Clone, PartialEq)]
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
#[cfg(test)]
mod tests {
    use crate::{color::Color, material::Material};

    #[test]
    fn the_default_material() {
        let m = Material::default();
        let c = Color::white();
        let ambient = 0.1;
        let diffuse = 0.9;
        let specular = 0.9;
        let shininess = 200.0;

        assert_eq!(m.color, c);
        assert_eq!(m.ambient, ambient);
        assert_eq!(m.diffuse, diffuse);
        assert_eq!(m.specular, specular);
        assert_eq!(m.shininess, shininess);
    }
}
