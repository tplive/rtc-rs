mod geometry;

fn main() {
    use crate::geometry::Point;

    let point = Point {
        x: 0.2,
        y: 0.3,
        z: 0.4,
        w: 1,
    };
    print!("WTF, {:?}", point);
}
