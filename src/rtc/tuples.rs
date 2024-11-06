use std::ops::Deref;

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
}
