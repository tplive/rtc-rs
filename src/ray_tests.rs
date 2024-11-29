use std::vec;

#[cfg(test)]
use crate::{
    ray::Ray,
    shape::{Intersectable, Intersection, Intersections, Shape, Sphere},
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

    let xs = s.intersect(r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);
}

#[test]
fn ray_intersects_sphere_at_tangent() {
    let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn ray_misses_sphere() {
    let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(r);

    assert_eq!(xs.len(), 0);
}

#[test]
fn ray_originates_inside_sphere() {
    let r = Ray::new(point(0.0, 0.0, -0.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);
}

#[test]
fn sphere_is_behind_ray() {
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
fn intersection_encapsulates_t_value_and_object() {
    let s = Sphere::new();
    let i = Intersection::new(3.5, Shape::Sphere(s));

    assert_eq!(i.t, 3.5);
    assert_eq!(i.shape, Shape::Sphere(s));
}

#[test]
fn aggregating_intersections() {
    let s = Sphere::new();
    let i1 = Intersection::new(1.0, Shape::Sphere(s));
    let i2 = Intersection::new(2.0, Shape::Sphere(s));
    let xs = vec![&i1, &i2];

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 1.0);
    assert_eq!(xs[1].t, 2.0);
}

#[test]
fn intersect_sets_the_object_on_the_intersection() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let xs = s.intersect(r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].shape, Shape::Sphere(s));
    assert_eq!(xs[1].shape, Shape::Sphere(s));
}

#[test]
fn the_hit_when_all_intersections_have_positive_t_value() {
    let s = Sphere::new();
    let i1 = Intersection::new(1.0, Shape::Sphere(s));
    let i2 = Intersection::new(2.0, Shape::Sphere(s));
    let xs = Intersections::new(vec![i2, i1]);

    let i = xs.hit();

    assert_eq!(i, Some(i1));
}
