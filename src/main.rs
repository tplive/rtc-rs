mod rtc;

fn main() {
    use crate::rtc::tuple::Point;

    let point = Point::new(0.2, 0.3, 0.4);
    print!("WTF, {:?}", point);
}
