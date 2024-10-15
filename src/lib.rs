
mod rtc;

#[cfg(test)]
mod tests {
    
    use crate::rtc::geometry::Point;

    #[test]
    fn create_point() {
        let p = Point::new(0.2, 0.3, -2.0);
        assert_eq!(p.w, 1);

        assert!(p.x == 0.2 && p.y == 0.3 && p.z == -2.0);
    }
}
