use crate::matrix::Matrix4;
use crate::ray::Ray;
#[cfg(test)]
use crate::shape::Intersectable;
use crate::shape::Sphere;
use crate::transformation::Transformation;
use crate::tuples::{point, vector};

#[test]
fn creating_a_sphere() {
    // Since id's are given by the global static function in utils.rs, and tests apparently run
    // async, there is no guarantee that the id's will be 1, 2, 3. Instead, we ensure that they are
    // unique and sequential.
    let s1 = Sphere::new(); // .id = n
    let s2 = Sphere::new(); // .id = n + 1
    let s3 = Sphere::new(); // .id = n + 2

    let n = s1.id;

    assert_eq!(s1.id, n);
    assert_eq!(s2.id, n + 1);
    assert_eq!(s3.id, n + 2);
}

#[test]
fn spheres_default_transform_is_identity_matrix() {
    let s = Sphere::new();
    let m = Matrix4::identity();

    assert_eq!(s.transform, m);
}

#[test]
fn changing_spheres_transform() {
    let mut s = Sphere::new();
    let t = Transformation::new().translation(2.0, 3.0, 4.0);

    s.transform = t.get();

    assert_eq!(s.transform, t.get());
}

#[test]
fn intersect_scaled_sphere_with_ray() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.transform = Transformation::new().scaling(2.0, 2.0, 2.0).get();

    let xs = s.intersect(r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
}

#[test]
fn intersect_translated_sphere_with_ray() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.transform = Transformation::new().translation(5.0, 0.0, 0.0).get();

    let xs = s.intersect(r);

    assert_eq!(xs.len(), 0);
}

