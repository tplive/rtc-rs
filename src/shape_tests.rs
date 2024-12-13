use std::f32::consts::PI;

#[cfg(test)]
use crate::{
    material::Material,
    matrix::Matrix4,
    ray::Ray,
    shape::{Intersectable, Sphere},
    transformation::{rotation_z, scaling, Transformation},
    tuples::{point, vector},
};

#[test]
fn creating_a_sphere() {
    // Since id's are given by the global static function in utils.rs, and tests apparently run
    // async, there is no guarantee that the id's will be 1, 2, 3. Instead, we ensure that they are
    // unique and sequential.
    let s1 = Sphere::default(); // .id = n
    let s2 = Sphere::default(); // .id = n + 1
    let s3 = Sphere::default(); // .id = n + 2

    let n = s1.id;

    assert_eq!(s1.id, n);
    assert_eq!(s2.id, n + 1);
    assert_eq!(s3.id, n + 2);
}

#[test]
fn spheres_default_transform_is_identity_matrix() {
    let s = Sphere::default();
    let m = Matrix4::identity();

    assert_eq!(s.transform, m);
}

#[test]
fn changing_spheres_transform() {
    let mut s = Sphere::default();
    let t = Transformation::new().translation(2.0, 3.0, 4.0);

    s.transform = t.get();

    assert_eq!(s.transform, t.get());
}

#[test]
fn intersect_scaled_sphere_with_ray() {
    let r = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
    let mut s = Sphere::default();
    s.transform = Transformation::new().scaling(2.0, 2.0, 2.0).get();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 3.0);
    assert_eq!(xs[1].t, 7.0);
}

#[test]
fn intersect_translated_sphere_with_ray() {
    let r = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
    let mut s = Sphere::default();
    s.transform = Transformation::new().translation(5.0, 0.0, 0.0).get();

    let xs = s.intersect(&r);

    assert_eq!(xs.len(), 0);
}

#[test]
fn normal_on_a_sphere_at_a_point_on_the_x_axis() {
    let s = Sphere::default();
    let n = s.normal_at(point(1.0, 0.0, 0.0));

    assert_eq!(vector(1.0, 0.0, 0.0), n);
}

#[test]
fn normal_on_a_sphere_at_a_point_on_the_y_axis() {
    let s = Sphere::default();
    let n = s.normal_at(point(0.0, 1.0, 0.0));

    assert_eq!(vector(0.0, 1.0, 0.0), n);
}

#[test]
fn normal_on_a_sphere_at_a_point_on_the_z_axis() {
    let s = Sphere::default();
    let n = s.normal_at(point(0.0, 0.0, 1.0));

    assert_eq!(vector(0.0, 0.0, 1.0), n);
}

#[test]
fn normal_on_a_sphere_at_a_non_axial_point() {
    let s = Sphere::default();
    let sqrt3over3 = 3.0_f32.sqrt() / 3.0;
    let n = s.normal_at(point(sqrt3over3, sqrt3over3, sqrt3over3));

    assert_eq!(vector(sqrt3over3, sqrt3over3, sqrt3over3), n);
}

#[test]
fn the_normal_is_a_normalized_vector() {
    let s = Sphere::default();
    let sqrt3over3 = 3.0_f32.sqrt() / 3.0;
    let n = s.normal_at(point(sqrt3over3, sqrt3over3, sqrt3over3));

    assert_eq!(n.normalize(), n);
}

#[test]
fn computing_normal_on_translated_sphere() {
    let mut s = Sphere::default();
    s.transform = Transformation::new().translation(0.0, 1.0, 0.0).get();

    let n = s.normal_at(point(0.0, 1.70711, -0.70711));

    assert_eq!(vector(0.0, 0.70711, -0.70711), n);
}

#[test]
fn computing_normal_on_transformed_sphere() {
    let mut s = Sphere::default();
    s.transform = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
    let sqrt2over2 = 2.0_f32.sqrt() / 2.0;

    let n = s.normal_at(point(0.0, sqrt2over2, -sqrt2over2));

    assert_eq!(vector(0.0, 0.97014, -0.24254), n);
}

#[test]
fn a_sphere_has_default_material() {
    let s = Sphere::default();
    let m = s.material;

    assert_eq!(m, Material::default());
}

#[test]
fn sphere_may_be_assigned_a_material() {
    let mut s = Sphere::default();
    let mut m = Material::default();
    m.shininess = 900.0;
    s.material = m;

    assert_eq!(s.material.shininess, 900.0);
}
