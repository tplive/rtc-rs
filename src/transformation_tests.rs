use std::f32::consts::PI;

#[cfg(test)]
use crate::{
    transformation::Transformation,
    tuples::{point, vector},
};

#[test]
fn multiplying_by_translation_matrix() {
    let transform = Transformation::translation(5.0, -3.0, 2.0);
    let p = point(-3.0, 4.0, 5.0);

    assert_eq!(point(2.0, 1.0, 7.0), transform * p);
}

#[test]
fn multiplying_by_inverse_of_translation_matrix() {
    let transform = Transformation::translation(5.0, -3.0, 2.0)
        .try_inverse()
        .unwrap();
    let p = point(-3.0, 4.0, 5.0);

    assert_eq!(point(-8.0, 7.0, 3.0), transform * p);
}

#[test]
fn translation_does_not_affect_vectors() {
    let transform = Transformation::translation(5.0, -3.0, 2.0)
        .try_inverse()
        .unwrap();
    let v = vector(-3.0, 4.0, 5.0);

    assert_eq!(vector(-3.0, 4.0, 5.0), transform * v);
}

#[test]
fn scaling_matrix_applied_to_point() {
    let transform = Transformation::scaling(2.0, 3.0, 4.0);
    let p = point(-4.0, 6.0, 8.0);
    //println!("{}", transform);

    assert_eq!(point(-8.0, 18.0, 32.0), transform * p);
}

#[test]
fn scaling_matrix_applied_to_vector() {
    let transform = Transformation::scaling(2.0, 3.0, 4.0);
    let v = vector(-4.0, 6.0, 8.0);
    //println!("{}", transform);

    assert_eq!(vector(-8.0, 18.0, 32.0), transform * v);
}

#[test]
fn multiply_by_inverse_of_scaling_matrix() {
    let transform = Transformation::scaling(2.0, 3.0, 4.0)
        .try_inverse()
        .unwrap();
    let v = vector(-4.0, 6.0, 8.0);
    //println!("{}", transform);

    assert_eq!(vector(-2.0, 2.0, 2.0), transform * v);
}

#[test]
fn reflection_is_scaling_by_negative_value() {
    let transform = Transformation::scaling(-1.0, 1.0, 1.0);
    let p = point(2.0, 3.0, 4.0);
    //println!("{}", transform);

    assert_eq!(point(-2.0, 3.0, 4.0), transform * p);
}

#[test]
fn rotating_point_around_x_axis() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = Transformation::rotation_x(PI / 4.0);
    let full_quarter = Transformation::rotation_x(PI / 2.0);

    let sqrt2div2 = 2.0_f32.sqrt() / 2.0;

    assert_eq!(half_quarter * p, point(0.0, sqrt2div2, sqrt2div2));
    assert_eq!(full_quarter * p, point(0.0, 0.0, 1.0,));
}

#[test]
fn inverse_of_x_rotation_goes_opposite_direction() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = Transformation::rotation_x(PI / 4.0).try_inverse().unwrap();

    let sqrt2div2 = 2.0_f32.sqrt() / 2.0;

    assert_eq!(half_quarter * p, point(0.0, sqrt2div2, -sqrt2div2));
}

#[test]
fn rotating_point_around_y_axis() {
    let p = point(0.0, 0.0, 1.0);
    let half_quarter = Transformation::rotation_y(PI / 4.0);
    let full_quarter = Transformation::rotation_y(PI / 2.0);

    let sqrt2div2 = 2.0_f32.sqrt() / 2.0;

    assert_eq!(half_quarter * p, point(sqrt2div2, 0.0, sqrt2div2));
    assert_eq!(full_quarter * p, point(1.0, 0.0, 0.0,));
}

#[test]
fn rotating_point_around_z_axis() {
    let p = point(0.0, 1.0, 0.0);
    let half_quarter = Transformation::rotation_z(PI / 4.0);
    let full_quarter = Transformation::rotation_z(PI / 2.0);

    let sqrt2div2 = 2.0_f32.sqrt() / 2.0;

    assert_eq!(half_quarter * p, point(-sqrt2div2, sqrt2div2, 0.0));
    assert_eq!(full_quarter * p, point(-1.0, 0.0, 0.0,));
}

#[test]
fn shearing_x_in_proportion_to_y() {
    let transform = Transformation::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, point(5.0, 3.0, 4.0));
}

#[test]
fn shearing_x_in_proportion_to_z() {
    let transform = Transformation::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, point(6.0, 3.0, 4.0));
}

#[test]
fn shearing_y_in_proportion_to_x() {
    let transform = Transformation::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, point(2.0, 5.0, 4.0));
}

#[test]
fn shearing_y_in_proportion_to_z() {
    let transform = Transformation::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let p = point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, point(2.0, 7.0, 4.0));
}

#[test]
fn shearing_z_in_proportion_to_x() {
    let transform = Transformation::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let p = point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, point(2.0, 3.0, 6.0));
}
#[test]
fn shearing_z_in_proportion_to_y() {
    let transform = Transformation::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let p = point(2.0, 3.0, 4.0);

    assert_eq!(transform * p, point(2.0, 3.0, 7.0));
}
