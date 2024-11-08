use std::ops;

const EPSILON: f32 = 0.00001_f32;

fn equal(a: f32, b: f32) -> bool {
    // Compare two f32 values for equality within the constant EPSILON
    (a - b).abs() < EPSILON
}

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
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

    fn add(self, other: Self) -> Self {
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

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::rtc::tuples::{point, vector, Tuple};

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
        let zero = vector(0.0,0.0,0.0);
        let v = vector(1.0, -2.0, 3.0);
        assert_eq!(vector(-1.0, 2.0, -3.0), zero - v);
    }

    #[test]
    fn negate_tuple() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(Tuple::new(-1.0, 2.0, -3.0, 4.0), -t);
    }
}
