use crate::{ray::Ray, tuples::point, util::RtcFl};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

type Intersection = [RtcFl; 2];

pub struct Sphere {
    pub id: usize,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b: f32 = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            return Some([t1, t2]);
        }
    }
}
