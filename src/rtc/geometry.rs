#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: u32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32)-> Self {
        Self {x, y, z, w: 1}
        
    }
}