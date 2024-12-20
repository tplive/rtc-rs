use std::ops;

use nalgebra::SMatrix;

use crate::{tuples::Tuple, util::{equal, RtcFl}};

/// We are of course using the nalgebra library instead of writing our own.
/// Been there, done that.
///
///
pub type Matrix2 = SMatrix<RtcFl, 2, 2>;
pub type Matrix3 = SMatrix<RtcFl, 3, 3>;
pub type Matrix4 = SMatrix<RtcFl, 4, 4>;

impl ops::Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut res: Vec<RtcFl> = vec![rhs.x, rhs.y, rhs.z, rhs.w];

        for r in 0..3 {
            res[r] = self[(r, 0)] * rhs.x
                + self[(r, 1)] * rhs.y
                + self[(r, 2)] * rhs.z
                + self[(r, 3)] * rhs.w;
        }

        Tuple::new(res[0], res[1], res[2], res[3])
    }
}

pub trait Submatrix<T> {
    fn submatrix(&self, row: usize, col: usize) -> T;
}

impl Submatrix<Matrix2> for Matrix3 {
    fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        let temp = &self.remove_row(row).remove_column(col);
        let m2x2: Matrix2 = temp.clone_owned();

        m2x2
    }
}

impl Submatrix<Matrix3> for Matrix4 {
    fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        let temp = &self.remove_row(row).remove_column(col);
        let m3x3: Matrix3 = temp.clone_owned();

        m3x3
    }
}

pub trait Operations<T> {
    fn minor(&self, row: usize, col: usize) -> RtcFl;
    fn cofactor(&self, row: usize, col: usize) -> RtcFl;
    fn equals(&self, other: T) -> bool;
}

impl Operations<Matrix3> for Matrix3 {
    fn minor(&self, row: usize, col: usize) -> RtcFl {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> RtcFl {
        let m = self.minor(row, col);
        if row + col % 2 == 0 {
            m
        } else {
            -m
        }
    }

    fn equals(&self, other: Matrix3) -> bool {
        let mut result = true;
        for n in 0..self.len() {
            if !equal(self[n], other[n]) {
                result = false;
            }
        }

        result
    }
}

impl Operations<Matrix4> for Matrix4 {
    fn minor(&self, row: usize, col: usize) -> RtcFl {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> RtcFl {
        let m = self.minor(row, col);
        if row + col % 2 == 0 {
            m
        } else {
            -m
        }
    }

    fn equals(&self, other: Matrix4) -> bool {
        let mut result = true;
        for n in 0..self.len() {
            if !equal(self[n], other[n]) {
                result = false;
            }
        }

        result
    }
}
