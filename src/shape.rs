use crate::{ray::Ray, tuples::point, util::RtcFl};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(PartialEq, Debug)]
pub enum Shape {
    Sphere(Sphere),
}

pub trait Intersectable {
    fn intersect(&self, ray: Ray) -> Vec<Intersection>;
}
// Inspired by MrJakob: https://youtu.be/lTrtsfYFTeE?si=niGyzutvTC_h92NY&t=965
impl Intersectable for Shape {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        match *self {
            Shape::Sphere(ref sphere) => sphere.intersect(ray),
        }
    }
}
pub struct Intersection {
    pub t: RtcFl,
    pub shape: Shape,
}

impl Intersection {
    pub fn new(t: RtcFl, shape: Shape) -> Self {
        Self { t, shape }
    }
}

pub struct Intersections {
    i: Vec<Intersection>,
}

impl Intersections {
    pub fn new(i: Vec<Intersection>) -> Self {
        Self { i }
    }
    /*
    pub fn hit(&self) -> Intersection {
        let mut sorted = self.i.sort_by(|a, b| a.t - b.t);
        let positives = sorted.map(|n| {
            if n.t >= 0.0 {
                return n;
            }
        });

        return positives[0];
    }
    */
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub id: usize,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b: f32 = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            return vec![
                Intersection::new(t1, Shape::Sphere(*self)),
                Intersection::new(t2, Shape::Sphere(*self)),
            ];
        }
    }
}
