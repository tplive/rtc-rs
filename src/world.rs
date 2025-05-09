use crate::color::Color;
use crate::computation::Computation;
use crate::light::{lighting, Light};
use crate::ray::Ray;
use crate::shape::{Intersection, Intersections, Shape};

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
    pub fn intersect<'w>(&'w self, ray: &Ray) -> Intersections<'w> {
        let xs: Vec<Intersection<'w>> = self
            .objects
            .iter()
            .flat_map(|shape_in_box| shape_in_box.intersect(ray))
            .collect();

        Intersections::new(xs)
    }

    pub fn add_object(&mut self, shape: impl Shape + 'static) {
        self.objects.push(Box::new(shape));
    }

    pub fn shade_hit(&self, comps: Computation) -> Color {
        lighting(
            &comps.shape.material(),
            &self.light[0],
            &comps.point,
            &comps.eyev,
            &comps.normalv,
        )
    }
}
