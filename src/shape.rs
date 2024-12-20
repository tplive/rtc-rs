use crate::{
    material::Material,
    matrix::Matrix4,
    ray::Ray,
    tuples::{point, Tuple},
    util::RtcFl,
};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Shape {
    Sphere(Sphere),
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
}
// Inspired by MrJakob: https://youtu.be/lTrtsfYFTeE?si=niGyzutvTC_h92NY&t=965
impl Intersectable for Shape {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        match *self {
            Shape::Sphere(ref sphere) => sphere.intersect(ray),
        }
    }
}

// TODO Can this be a part of Intersectable?
pub trait NormalAt {
    fn normal_at(&self, p: Tuple) -> Tuple;
}

impl NormalAt for Shape {
    fn normal_at(&self, p: Tuple) -> Tuple {
        match *self {
            Shape::Sphere(ref sphere) => sphere.normal_at(p),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    data: Vec<Intersection>,
}

impl Intersections {
    pub fn new(mut data: Vec<Intersection>) -> Self {
        data.sort_unstable_by(|a, b| {
            a.t.partial_cmp(&b.t)
                .expect("Unable to sort intersections!")
        });
        Self { data }
    }

    pub fn hit(&self) -> Option<Intersection> {
        for n in self.data.iter() {
            if n.t >= 0.0 {
                return Some(*n);
            }
        }

        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub id: usize,
    pub transform: Matrix4,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            transform: Matrix4::identity(),
            material: Material::default(),
        }
    }
}

impl Sphere {
    pub fn new(transform: Matrix4, material: Material) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            transform,
            material,
        }
    }

    pub fn normal_at(&self, p: Tuple) -> Tuple {
        let inverse_transform = match self.transform.try_inverse() {
            Some(matrix) => matrix,
            None => {
                println!("{}", self.transform);
                panic!("Cannot invert matrix.");
            }
        };

        let object_point = inverse_transform * p;
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let world_normal = inverse_transform.transpose() * object_normal;

        Tuple::new(world_normal.x, world_normal.y, world_normal.z, 0.0).normalize()
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let transformed_ray = ray.transform(
            self.transform
                .try_inverse()
                .expect("Cannot invert this transform."),
        );

        let sphere_to_ray = transformed_ray.origin - point(0.0, 0.0, 0.0);
        let a = transformed_ray.direction.dot(transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            
            vec![
                Intersection::new(t1, Shape::Sphere(*self)),
                Intersection::new(t2, Shape::Sphere(*self)),
            ]
        }
    }
}
