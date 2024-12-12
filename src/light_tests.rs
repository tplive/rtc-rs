use crate::material::Material;
#[cfg(test)]
use crate::{
    color::Color,
    light::{Light, lighting},
    tuples::{point, vector},
};

#[test]
fn a_point_light_has_position_and_intensity() {
    let intensity = Color::new(1.0, 1.0, 1.0);
    let position = point(0.0, 0.0, 0.0);
    let light = Light::point(position, intensity);

    assert!(light.position.eq(&position));
    assert!(light.intensity.eq(&intensity));
}

#[test]
fn lighting_with_eye_between_light_and_surface() {
    let m = Material::default();
    let position = point(0.0, 0.0, 0.0);

    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light::point(point(0.0, 0.0, -10.0), Color::white());
    let result = lighting(m, light, position, eyev, normalv);

    assert!(Color::new(1.9, 1.9, 1.9).eq(&result));
}

#[test]
fn lighting_with_eye_between_light_and_surface_eye_offset_45_degrees() {
    let m = Material::default();
    let sqrt2over2 = 2.0_f32.sqrt() / 2.0;
    let position = point(0.0, 0.0, 0.0);

    let eyev = point(0.0, sqrt2over2, sqrt2over2);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light::point(point(0.0, 0.0, -10.0), Color::white());
    let result = lighting(m, light, position, eyev, normalv);

    //println!("{:?}", &result);
    assert!(Color::new(1.0, 1.0, 1.0).eq(&result));
}

#[test]
fn lighting_with_eye_oposite_surface_light_offset_45_degrees() {
    let m = Material::default();
    let sqrt2over2 = 2.0_f32.sqrt() / 2.0;
    let position = point(0.0, 0.0, 0.0);

    let eyev = point(0.0, sqrt2over2, sqrt2over2);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light::point(point(0.0, 10.0, -10.0), Color::white());
    let result = lighting(m, light, position, eyev, normalv);
    
    //println!("{:?}", &result);
    assert!(Color::new(0.7364, 0.7364, 0.7364).eq(&result));
}

#[test]
fn lighting_with_eye_in_path_of_reflection_vector() {
    let m = Material::default();
    let sqrt2over2 = 2.0_f32.sqrt() / 2.0;
    let position = point(0.0, 0.0, 0.0);

    let eyev = point(0.0, -sqrt2over2, -sqrt2over2);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light::point(point(0.0, 10.0, -10.0), Color::white());
    let result = lighting(m, light, position, eyev, normalv);
    
    //println!("{:?}", &result);
    assert!(Color::new(1.6364, 1.6364, 1.6364).eq(&result));
}


#[test]
fn lighting_with_light_behind_surface() {
    let m = Material::default();
    let position = point(0.0, 0.0, 0.0);

    let eyev = point(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = Light::point(point(0.0, 0.0, 10.0), Color::white());
    let result = lighting(m, light, position, eyev, normalv);
    
    //println!("{:?}", &result);
    assert!(Color::new(0.1, 0.1, 0.1).eq(&result));
}
