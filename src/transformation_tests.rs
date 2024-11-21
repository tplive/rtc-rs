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

