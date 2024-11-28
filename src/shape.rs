use crate::{ray::Ray, tuples::point, util::RtcFl};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

pub struct Intersection<'a> {
    pub t: RtcFl,
    pub shape: &'a Sphere,
}

impl <'a> Intersection<'a> {
    pub fn new(t: RtcFl, shape: &'a Sphere) -> Self {
        Self {
            t,
            shape,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub id: usize,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b: f32 = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec!();
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            return vec![Intersection::new(t1, &self), Intersection::new(t2, &self)];
        }
    }
}
