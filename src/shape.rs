use crate::{
    intersections::Intersection,
    material::Material,
    matrix::Matrix4,
    ray::Ray,
    tuples::Tuple,
};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

pub fn next_shape_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

pub trait Shape: Send + Sync + std::fmt::Debug {
    fn intersect<'s>(&'s self, ray: &Ray) -> Vec<Intersection<'s>>;
    fn normal_at(&self, world_point: Tuple) -> Tuple;
    fn material(&self) -> &Material;
    fn transform(&self) -> &Matrix4;
    fn id(&self) -> usize;
    fn clone_boxed(&self) -> Box<dyn Shape>;
}

#[cfg(test)]
mod tests {

    use crate::{matrix::Matrix4, shape::Shape, sphere::Sphere};

    #[test]
    fn the_default_transformation() {
        let s = Sphere::default();

        assert_eq!(*s.transform(), Matrix4::identity());
    }
}
