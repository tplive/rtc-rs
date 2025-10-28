use crate::{color::Color, pattern::Pattern, util::RtcFl};

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub pattern: Option<Pattern>,
    pub ambient: RtcFl,
    pub diffuse: RtcFl,
    pub specular: RtcFl,
    pub shininess: RtcFl,
}

impl Material {
    pub fn new(
        color: Color,
        pattern: Option<Pattern>,
        ambient: RtcFl,
        diffuse: RtcFl,
        specular: RtcFl,
        shininess: RtcFl,
    ) -> Self {
        Self {
            color,
            pattern,
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
            pattern: None,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}
#[cfg(test)]
mod tests {

    use crate::{
        color::Color,
        light::{lighting, Light},
        material::Material,
        pattern::{Pattern, StripePattern},
        tuples::{point, vector},
    };

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

    #[test]
    fn lighting_with_pattern_applied() {
        let mut m = Material::default();
        m.pattern = Some(Pattern::Stripe(StripePattern::new(
            Color::white(),
            Color::black(),
        )));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::point(point(0.0, 0.0, -10.0), Color::white());

        let c1 = lighting(&m, &light, &point(0.9, 0.0, 0.0), &eyev, &normalv, false);
        let c2 = lighting(&m, &light, &point(1.1, 0.0, 0.0), &eyev, &normalv, false);

        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }
}
