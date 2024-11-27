#[cfg(test)]
use crate::{
    ray::Ray,
    shape::Sphere,
    tuples::{point, vector},
};

#[test]
fn creating_and_querying_a_ray() {
    let origin = point(1.0, 2.0, 3.0);
    let direction = vector(4.0, 5.0, 6.0);

    let ray = Ray::new(origin, direction);

    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
}

#[test]
fn computing_a_point_from_a_distance() {
    let r = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));

    assert_eq!(r.position(0.0), point(2.0, 3.0, 4.0));
    assert_eq!(r.position(1.0), point(3.0, 3.0, 4.0));
    assert_eq!(r.position(-1.0), point(1.0, 3.0, 4.0));
    assert_eq!(r.position(2.5), point(4.5, 3.0, 4.0));
}

#[test]
fn ray_intersects_sphere_to_two_points() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = r.intersect(s);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);
}

#[test]
fn ray_intersects_sphere_at_tangent() {
    let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = r.intersect(s);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0], 5.0);
    assert_eq!(xs[1], 5.0);
}

#[test]
fn ray_misses_sphere() {
    let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = r.intersect(s);

    assert_eq!(xs.count(), 0);
}

#[test]
fn ray_originates_inside_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = r.intersect(s);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0], -1.0);
    assert_eq!(xs[1], 1.0);
}

#[test]
fn sphere_is_behind_ray() {
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = r.intersect(s);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs[0], -6.0);
    assert_eq!(xs[1], 4.0);
}
