#[cfg(test)]
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
