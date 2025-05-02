#[cfg(test)]
use crate::color::Color;
use crate::light::Light;
use crate::shape::Sphere;
use crate::transformation::{Transformation, scaling};
use crate::tuples::point;
use crate::world::World;

#[test]
fn creating_a_world() {
    let w = World::default();

    assert!(w.objects.is_empty());
    assert!(w.light.is_empty());
}

#[test]
fn the_default_world() {
    let w = World::default();
    let wli = w.light[0].intensity;
    let wlp = w.light[0].position;
    
    let light = Light::point(point(-10.0, 10.0, -10.0), Color::white());
    let s1 = Sphere::default();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let s2 = Sphere::default();
    s2.transform = scaling(0.5, 0.5, 0.5);

    assert!(w.objects.contains(s1) && w.objects.contains(s2));

    assert!(w.light[0].intensity == Color::new(1.0, 1.0, 1.0));
    assert!(w.light[0].position == point(-10.0, 10.0, -10.0));

    
}
