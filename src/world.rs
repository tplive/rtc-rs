use crate::ray::Ray;
use crate::shape::{Intersections, Shape};
use crate::light::Light;

pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
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

impl World {
    pub fn intersect(&self, ray: Ray) -> Intersections {

        let mut all_intersections = vec![];

        // Iterate over all objects in self, and check intersections
        for shape in &self.objects {
            let shape_intersections = shape.intersect(&ray);
            all_intersections.extend(shape_intersections);
        }

        Intersections::new(all_intersections)
    }
}