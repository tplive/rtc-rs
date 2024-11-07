use std::ops;

const EPSILON: f32 = 0.00001_f32;

fn equal(a: f32, b: f32) -> bool {
    // Compare two f32 values for equality within the constant EPSILON
    (a - b).abs() < EPSILON
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn can_add_two_points() {
        let p1 = point(1.0, 2.0, 3.0);
        let p2 = point(3.0, 2.0, 1.0);
        let expected_result = point(4.0, 4.0, 4.0);
        let actual_result = p1 + p2;
        assert_eq!(expected_result, actual_result);
    }
}
