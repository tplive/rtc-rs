#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: u32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1 }
    }
}

#[cfg(test)]
mod tests {

    use crate::rtc::tuple::Point;

    #[test]
    fn create_point() {
        let p = Point::new(0.2, 0.3, -2.0);
        assert_eq!(p.w, 1);

        assert!(p.x == 0.2 && p.y == 0.3 && p.z == -2.0);
    }
}
