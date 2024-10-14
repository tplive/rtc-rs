

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
    w: u32,
}

fn main() {
    let point = Point {
        x: 0.2,
        y: 0.3,
        z: 0.4,
        w: 1,
    };
    print!("WTF, {:?}", point);
}

#[test]
fn create_point() {
    let point = Point {
        x: 0.2,
        y: 0.3,
        z: 0.4,
        w: 1,
    };
    assert!(point.x == 0.2 && point.y == 0.3 && point.z == 0.4 && point.w == 1);
}
