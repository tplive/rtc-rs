#[cfg(test)]
use crate::{color::Color, light::Light, tuples::point};

#[test]
fn a_point_light_has_position_and_intensity() {
    let intensity = Color::new(1.0, 1.0, 1.0);
    let position = point(0.0, 0.0, 0.0);
    let light = Light::point(position, intensity);

    assert!(light.position.eq(&position));
    assert!(light.intensity.eq(&intensity));
}
