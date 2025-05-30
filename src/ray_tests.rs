use std::vec;

use crate::transformation::Transformation;
#[cfg(test)]
use crate::{
    intersections::{Intersection, Intersections},
    ray::Ray,
    shape::Shape,
    sphere::Sphere,
    tuples::{point, vector},
};

#[test]
fn creating_and_querying_a_ray() {
    let origin = point(1.0, 2.0, 3.0);
    let direction = vector(4.0, 5.0, 6.0);

    let ray = Ray::new(&origin, &direction);

    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
}

#[test]
fn computing_a_point_from_a_distance() {
    let r = Ray::new(&point(2.0, 3.0, 4.0), &vector(1.0, 0.0, 0.0));

    assert_eq!(r.position(0.0), point(2.0, 3.0, 4.0));
    assert_eq!(r.position(1.0), point(3.0, 3.0, 4.0));
    assert_eq!(r.position(-1.0), point(1.0, 3.0, 4.0));
    assert_eq!(r.position(2.5), point(4.5, 3.0, 4.0));
}

#[test]
fn ray_intersects_sphere_to_two_points() {
    let r = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
    let s = Sphere::default();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 4.0);
    assert_eq!(xs[1].t, 6.0);
}

#[test]
fn ray_intersects_sphere_at_tangent() {
    let r = Ray::new(&point(0.0, 1.0, -5.0), &vector(0.0, 0.0, 1.0));
    let s = Sphere::default();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 5.0);
    assert_eq!(xs[1].t, 5.0);
}

#[test]
fn ray_misses_sphere() {
    let r = Ray::new(&point(0.0, 2.0, -5.0), &vector(0.0, 0.0, 1.0));
    let s = Sphere::default();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 0);
}

#[test]
fn ray_originates_inside_sphere() {
    let r = Ray::new(&point(0.0, 0.0, -0.0), &vector(0.0, 0.0, 1.0));
    let s = Sphere::default();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -1.0);
    assert_eq!(xs[1].t, 1.0);
}

#[test]
fn sphere_is_behind_ray() {
    let r = Ray::new(&point(0.0, 0.0, 5.0), &vector(0.0, 0.0, 1.0));
    let s = Sphere::default();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, -6.0);
    assert_eq!(xs[1].t, -4.0);
}

#[test]
fn intersection_encapsulates_t_value_and_object() {
    let s = Sphere::default();
    let i = Intersection::new(3.5, &s);

    assert_eq!(i.t, 3.5);
    assert_eq!(i.shape.id(), s.id());
}

#[test]
fn aggregating_intersections() {
    let s = Sphere::default();
    let i1 = Intersection::new(1.0, &s);
    let i2 = Intersection::new(2.0, &s);
    let xs_data_for_creation = vec![i1, i2];
    let xs = Intersections::new(xs_data_for_creation);

    assert_eq!(xs.data.len(), 2);
    assert_eq!(xs.data[0].t, 1.0);
    assert_eq!(xs.data[0].shape.id(), s.id());
    assert_eq!(xs.data[1].t, 2.0);
    assert_eq!(xs.data[1].t, 2.0);
    assert_eq!(xs.data[1].shape.id(), s.id());
}

#[test]
fn intersect_sets_the_object_on_the_intersection() {
    let r = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
    let s = Sphere::default();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].shape.id(), s.id());
    assert_eq!(xs[1].shape.id(), s.id());
}

#[test]
fn the_hit_when_all_intersections_have_positive_t_value() {
    let s = Sphere::default();
    let i1 = Intersection::new(1.0, &s);
    let i2 = Intersection::new(2.0, &s);
    let xs = Intersections::new(vec![i2, i1]);

    let i = xs.hit();

    assert_eq!(i, Some(i1));
}

#[test]
fn the_hit_when_some_intersections_have_negative_t_value() {
    let s = Sphere::default();
    let i1 = Intersection::new(-1.0, &s);
    let i2 = Intersection::new(1.0, &s);
    let xs = Intersections::new(vec![i2, i1]);

    let i = xs.hit();

    assert_eq!(i, Some(i2));
}

#[test]
fn the_hit_when_all_intersections_have_negative_t_value() {
    let s = Sphere::default();
    let i1 = Intersection::new(-2.0, &s);
    let i2 = Intersection::new(-1.0, &s);
    let xs = Intersections::new(vec![i2, i1]);

    let i = xs.hit();

    assert_eq!(i, None);
}

#[test]
fn the_hit_is_always_the_lowest_nonnegative_intersection() {
    let s = Sphere::default();
    let i1 = Intersection::new(5.0, &s);
    let i2 = Intersection::new(7.0, &s);
    let i3 = Intersection::new(-3.0, &s);
    let i4 = Intersection::new(2.0, &s);
    let xs = Intersections::new(vec![i1, i2, i3, i4]);

    let i = xs.hit();

    assert_eq!(i, Some(i4));
}

#[test]
fn translating_a_ray() {
    let r1 = Ray::new(&point(1.0, 2.0, 3.0), &vector(0.0, 1.0, 0.0));
    let m = Transformation::new().translation(3.0, 4.0, 5.0);
    let r2 = r1.transform(m.get());

    assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
    assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
}

#[test]
fn scaling_a_ray() {
    let r1 = Ray::new(&point(1.0, 2.0, 3.0), &vector(0.0, 1.0, 0.0));
    let m = Transformation::new().scaling(2.0, 3.0, 4.0);
    let r2 = r1.transform(m.get());

    assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
    assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
}
