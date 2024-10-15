mod rtc;

fn main() {
    use crate::rtc::geometry::Point;

    let point = Point::new(0.2, 0.3, 0.4);
    print!("WTF, {:?}", point);
}
