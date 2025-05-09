use crate::{
    intersections::Intersection,
    material::Material,
    matrix::Matrix4,
    ray::Ray,
    tuples::{point, Tuple},
};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

pub trait Shape: std::fmt::Debug {
    fn intersect<'s>(&'s self, ray: &Ray) -> Vec<Intersection<'s>>;
    fn normal_at(&self, world_point: Tuple) -> Tuple;
    fn material(&self) -> &Material;
    fn transform(&self) -> &Matrix4;
    fn id(&self) -> usize;
    fn clone_boxed(&self) -> Box<dyn Shape>;
}

#[derive(Debug, Copy, Clone)]
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

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.material == other.material && self.transform == other.transform
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
}

impl Shape for Sphere {
    fn intersect<'s>(&'s self, ray: &Ray) -> Vec<Intersection<'s>> {
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

            vec![Intersection::new(t1, self), Intersection::new(t2, self)]
        }
    }

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let inverse_transform = match self.transform.try_inverse() {
            Some(matrix) => matrix,
            None => {
                println!("{}", self.transform);
                panic!("Cannot invert matrix.");
            }
        };

        //println!("Inverse transform: {:?}", inverse_transform);

        let object_point = inverse_transform * world_point;
        //println!("Object point: {:?}", object_point);
        let object_normal = (object_point - point(0.0, 0.0, 0.0)).normalize();
        //println!("Object normal: {:?}", object_normal);

        let mut world_normal = inverse_transform.transpose() * object_normal;
        //println!("World normal before normalization: {:?}", world_normal);

        // Hack to reset the w component, avoiding some more complex matrix math
        world_normal.w = 0.0;

        world_normal.normalize()
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn id(&self) -> usize {
        self.id
    }

    fn clone_boxed(&self) -> Box<dyn Shape> {
        Box::new(*self)
    }
}
