use std::ops::Deref;

const EPSILON:f32 = 0.00001_f32;

fn equal(a: f32, b: f32) -> bool {
    // Compare two f32 values for equality within the constant EPSILON
    (a - b).abs() < EPSILON
}

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    w: f32,
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

#[derive(Debug, Clone, Copy)]
pub struct Point {
    tuple: Tuple,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    tuple: Tuple,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { tuple: Tuple::new(x, y, z, 1.0) }
    }
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { tuple: Tuple::new(x, y, z, 0.0) }
    }
}

impl Deref for Point {
    type Target = Tuple;
    fn deref(&self) -> &Self::Target {
        &self.tuple
    }
}

impl Deref for Vector {
    type Target = Tuple;
    fn deref(&self) -> &Self::Target {
        &self.tuple
    }
}


#[cfg(test)]
mod tests {

    use crate::rtc::tuples::{Point, Vector};

    use super::Tuple;

    #[test]
    fn test_point() {
        let p = Point::new(0.2, 0.3, -2.0);
        assert_eq!(p.w, 1.0); // Ensure w is 1 for points.

        assert!(p.x == 0.2 && p.y == 0.3 && p.z == -2.0); // Ensure values match the input.
    }

    #[test]
    fn test_vector() {
        let v = Vector::new(-0.4, 1.1, 2.0);
        assert_eq!(v.w, 0.0); // Ensure w is 0 for vectors.
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
        let p = Point::new(4.0, -4.0, 3.0);
        assert_eq!(p.w, 1.0);
        assert!(p.is_point());
        assert!(!p.is_vector());
    }
    
    #[test]
    fn vector_creates_tuple_with_w_0_0() {
        let v = Vector::new(4.0, -4.0, 3.0);
        assert_eq!(v.w, 0.0);
        assert!(!v.is_point());
        assert!(v.is_vector());
    }
}
