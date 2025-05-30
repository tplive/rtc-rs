use std::ops;

use crate::util::{equal, RtcFl};

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: RtcFl,
    pub y: RtcFl,
    pub z: RtcFl,
    pub w: RtcFl,
}

impl Tuple {
    pub fn new(x: RtcFl, y: RtcFl, z: RtcFl, w: RtcFl) -> Self {
        Self { x, y, z, w }
    }

    pub fn is_point(&self) -> bool {
        equal(self.w, 1.0)
    }

    pub fn is_vector(&self) -> bool {
        equal(self.w, 0.0)
    }

    pub fn mag(&self) -> RtcFl {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.mag();
        Self::new(self.x / m, self.y / m, self.z / m, self.w / m)
    }

    pub fn dot(&self, other: Self) -> RtcFl {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
            0.0,
        )
    }

    pub fn reflect(self, normal: Tuple) -> Self {
        self - normal * 2.0 * self.dot(normal)
    }
}

pub fn point(x: RtcFl, y: RtcFl, z: RtcFl) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: RtcFl, y: RtcFl, z: RtcFl) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        equal(self.x, other.x)
            && equal(self.y, other.y)
            && equal(self.z, other.z)
            && equal(self.w, other.w)
    }
}

impl ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl ops::Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl ops::Mul<RtcFl> for Tuple {
    type Output = Self;

    fn mul(self, scalar: RtcFl) -> Self::Output {
        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl ops::Div<RtcFl> for Tuple {
    type Output = Self;

    fn div(self, scalar: RtcFl) -> Self::Output {
        Self::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::{point, vector, Tuple};
    use crate::util::{equal, RtcFl};

    #[test]
    fn test_point() {
        let p = point(0.2, 0.3, -2.0);
        assert_eq!(p.w, 1.0); // Ensure w is 1 for points.
        assert!(p.is_point());
        assert!(!p.is_vector());
        assert!(p.x == 0.2 && p.y == 0.3 && p.z == -2.0); // Ensure values match the input.
    }

    #[test]
    fn test_vector() {
        let v = vector(-0.4, 1.1, 2.0);
        assert_eq!(v.w, 0.0); // Ensure w is 0 for vectors.
        assert!(v.is_vector());
        assert!(!v.is_point());
        assert!(v.x == -0.4 && v.y == 1.1 && v.z == 2.0); // Ensure values match the input.
    }

    #[test]
    fn a_tuple_with_w_1_0_is_a_point() {
        // This is the test from p. 4 of the book.

        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn a_tuple_with_w_0_0_is_a_vector() {
        // This is the test from p. 4 of the book.

        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuple_with_w_1_0() {
        let p = point(4.0, -4.0, 3.0);
        assert_eq!(p.w, 1.0);
        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn vector_creates_tuple_with_w_0_0() {
        let v = vector(4.0, -4.0, 3.0);
        assert_eq!(v.w, 0.0);
        assert!(!v.is_point());
        assert!(v.is_vector());
    }

    #[test]
    fn adding_two_tuples() {
        // Test from p. 6 in the book
        let p = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let v = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let expected_result = Tuple::new(1.0, 1.0, 6.0, 1.0);
        let actual_result = p + v;
        assert!(expected_result.eq(&actual_result));
    }

    #[test]
    fn subtract_two_points() {
        // Test from p. 6 in the book
        let p = point(3.0, 2.0, 1.0);
        let v = point(5.0, 6.0, 7.0);
        let expected_result = vector(-2.0, -4.0, -6.0);
        let actual_result = p - v;
        assert!(expected_result.eq(&actual_result));
    }

    #[test]
    fn adding_two_points_should_fail() {
        let p1 = point(1.0, 2.0, 3.0);
        let p2 = point(3.0, 2.0, 1.0);
        assert_ne!(point(4.0, 4.0, 4.0), p1 + p2);
    }

    #[test]
    fn subtract_vector_from_point() {
        // Test from p. 6
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(point(-2.0, -4.0, -6.0), p - v);
    }

    #[test]
    fn subtract_two_vectors() {
        // Test from p. 7
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_eq!(vector(-2.0, -4.0, -6.0), v1 - v2);
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        // Test from p. 7
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        assert_eq!(vector(-1.0, 2.0, -3.0), zero - v);
    }

    #[test]
    fn negate_tuple() {
        // Test from p. 7
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(Tuple::new(-1.0, 2.0, -3.0, 4.0), -t);
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        // Test from p. 8
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let s: RtcFl = 3.5;
        let expected = Tuple::new(3.5, -7.0, 10.5, -14.0);
        let actual = t * s;
        assert!(actual.eq(&expected));
    }
    #[test]
    fn multiply_tuple_by_fraction() {
        // Test from p. 8
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let s = 0.5;
        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        let actual = t * s;
        assert!(actual.eq(&expected));
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        // Test from p. 8
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let s = 2.0;
        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        let actual = t / s;
        assert!(actual.eq(&expected));
    }

    #[test]
    fn compute_magnitude_of_vector() {
        let v1 = vector(1.0, 0.0, 0.0);
        assert_eq!(v1.mag(), 1.0);

        let v2 = vector(0.0, 1.0, 0.0);
        assert_eq!(v2.mag(), 1.0);

        let v3 = vector(0.0, 0.0, 1.0);
        assert_eq!(v3.mag(), 1.0);

        let v4 = vector(1.0, 2.0, 3.0);
        assert!(equal(v4.mag(), 14.0_f32.sqrt() as RtcFl)); // Hacky, to accomodated RtcFl

        let v5 = vector(-1.0, -2.0, -3.0);
        assert!(equal(v5.mag() as RtcFl, 14.0_f32.sqrt() as RtcFl)); // Hacky, to accomodated RtcFl
    }

    #[test]
    fn normalize_vector() {
        let v1 = vector(4.0, 0.0, 0.0);
        let exp1 = vector(1.0, 0.0, 0.0);
        assert_eq!(v1.normalize(), exp1);

        let v2 = vector(1.0, 2.0, 3.0);
        let exp2 = vector(0.26726124, 0.5345225, 0.8017837);
        assert_eq!(v2.normalize(), exp2);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert!(v1.dot(v2) == 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        let expected1 = vector(-1.0, 2.0, -1.0);
        let expected2 = vector(1.0, -2.0, 1.0);

        assert_eq!(v1.cross(v2), expected1);
        assert_eq!(v2.cross(v1), expected2);
    }

    #[test]
    fn reflect_vector_approaching_at_45_degrees() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = v.reflect(n);

        assert_eq!(r, vector(1.0, 1.0, 0.0));
    }
    #[test]
    fn reflect_vector_off_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let sqrt2over2 = 2.0_f32.sqrt() / 2.0;

        let n = vector(sqrt2over2, sqrt2over2, 0.0);
        let r = v.reflect(n);

        assert_eq!(r, vector(1.0, 0.0, 0.0));
    }
}
