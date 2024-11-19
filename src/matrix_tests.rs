#[cfg(test)]
use nalgebra::RowVector4;

use crate::{matrix::{Matrix2x2, Matrix3x3, Matrix4x4}, tuples::Tuple};

#[test]
fn constructing_and_inspecting_a_4x4_matrix() {
    let m = Matrix4x4::new(
        1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
    );

    assert_eq!(m[(0, 0)], 1.0);
    assert_eq!(m[(0, 3)], 4.0);
    assert_eq!(m[(1, 2)], 7.5);
    assert_eq!(m[(2, 2)], 11.0);
    assert_eq!(m[(3, 0)], 13.5);
    assert_eq!(m[(3, 2)], 15.5);
}

#[test]
fn can_make_2x2_matrix() {
    let m = Matrix2x2::new(-3.0, 5.0, 1.0, -2.0);

    assert_eq!(m[(0, 0)], -3.0);
    assert_eq!(m[(0, 1)], 5.0);
    assert_eq!(m[(1, 0)], 1.0);
    assert_eq!(m[(1, 1)], -2.0);
}

#[test]
fn can_make_3x32_matrix() {
    let m = Matrix3x3::new(-3.0, 5.0, 0.0, 1.0, -2.0, -1.0, 0.0, 1.0, 1.0);

    assert_eq!(m[(0, 0)], -3.0);
    assert_eq!(m[(0, 1)], 5.0);
    assert_eq!(m[(1, 1)], -2.0);
}

#[test]
fn matrix_equality() {
    let mut ma = Matrix4x4::identity();

    ma.set_row(0, &RowVector4::new(1.0, 2.0, 3.0, 4.0));
    ma.set_row(1, &RowVector4::new(5.0, 6.0, 7.0, 8.0));
    ma.set_row(2, &RowVector4::new(9.0, 8.0, 7.0, 6.0));
    ma.set_row(3, &RowVector4::new(5.0, 4.0, 3.0, 2.0));

    let mb = Matrix4x4::new(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
    );

    assert!(ma == mb);
}

#[test]
fn matrix_inequality() {
    let mut ma = Matrix4x4::identity();

    ma.set_row(0, &RowVector4::new(1.0, 2.0, 3.0, 4.0));
    ma.set_row(1, &RowVector4::new(5.0, 6.0, 7.0, 8.0));
    ma.set_row(2, &RowVector4::new(9.0, 8.0, 7.0, 6.0));
    ma.set_row(3, &RowVector4::new(5.0, 4.0, 3.0, 2.0));

    let mb = Matrix4x4::new(
        2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
    );

    assert!(ma != mb);
}

#[test]
fn multiply_matrices() {
    let mut ma = Matrix4x4::identity();

    ma.set_row(0, &RowVector4::new(1.0, 2.0, 3.0, 4.0));
    ma.set_row(1, &RowVector4::new(5.0, 6.0, 7.0, 8.0));
    ma.set_row(2, &RowVector4::new(9.0, 8.0, 7.0, 6.0));
    ma.set_row(3, &RowVector4::new(5.0, 4.0, 3.0, 2.0));

    let mb = Matrix4x4::new(
        -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
    );

    let m_actual = ma * mb;
    let m_expected = Matrix4x4::new(
        20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0,
        46.0, 42.0,
    );

    assert_eq!(m_actual, m_expected);
}

#[test]
fn matrix_multiplied_by_tuple() {
    let A = Matrix4x4::new(
        1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
    );

    let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

    let t_actual = A * b;
    let t_expected = Tuple::new(18.0, 24.0, 33.0, 1.0);

    assert_eq!(t_actual, t_expected);
}
