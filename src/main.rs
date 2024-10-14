#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
    w: u32,
}

fn main() {
    let point = Point{x: 0.2, y:0.3, z:0.4, w:1};
    print!("WTF, {:?}", point);
}
