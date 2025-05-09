#[cfg(test)]
use crate::color::Color;
use crate::light::Light;
use crate::ray::Ray;
use crate::shape::Sphere;
use crate::transformation::scaling;
use crate::tuples::{point, vector};
use crate::world::World;

#[test]
fn creating_a_world() {
    let w = World::default();

    assert!(w.objects.is_empty());
    assert!(w.light.is_empty());
}

fn create_default_world_for_test() -> World {

    let light = Light::point(point(-10.0, 10.0, -10.0), Color::white());

    let mut s1_created = Sphere::default();
    s1_created.material.color = Color::new(0.8, 1.0, 0.6);
    s1_created.material.diffuse = 0.7;
    s1_created.material.specular = 0.2;

    let mut s2_created = Sphere::default();
    s2_created.transform = scaling(0.5, 0.5, 0.5);

    let mut world = World::default(); 
    world.add_object(s1_created);
    world.add_object(s2_created);
    world.light.push(light);

    world
}

#[test]
fn the_default_world() {
    let w = create_default_world_for_test();

    let mut s1 = Sphere::default();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let mut s2 = Sphere::default();
    s2.transform = scaling(0.5, 0.5, 0.5);

    assert!(w.light.len() == 1);
    assert!(w.light[0].intensity == Color::new(1.0, 1.0, 1.0));
    assert!(w.light[0].position == point(-10.0, 10.0, -10.0));

    assert_eq!(w.objects.len(), 2);
    assert!(w.objects.iter().any(|shape| shape.material() == &s1.material && shape.transform() == &s1.transform ));
    assert!(w.objects.iter().any(|shape| shape.material() == &s2.material && shape.transform() == &s2.transform ));

}

#[test]
fn intersect_world_with_ray() {
    let w = create_default_world_for_test();
    let r = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));

    let xs = w.intersect(&r);

    assert!(xs.data.len() == 4);
    assert!(xs.data[0].t == 4.0);
    assert!(xs.data[1].t == 4.5);
    assert!(xs.data[2].t == 5.5);
    assert!(xs.data[3].t == 6.0);
}