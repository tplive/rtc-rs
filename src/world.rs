use crate::shape::Shape;
use crate::light::Light;

pub struct World {
    pub objects: Vec<Shape>,
    pub light: Vec<Light>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
            light: Vec::new(),
        }
    }
}
