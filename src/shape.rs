use crate::{
    intersections::Intersection,
    material::Material,
    matrix::Matrix4,
    ray::Ray,
    tuples::{point, Tuple},
};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

pub trait Shape: Send + Sync + std::fmt::Debug {
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

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    #[cfg(test)]
    use crate::{
        material::Material,
        matrix::Matrix4,
        ray::Ray,
        shape::{Shape, Sphere},
        transformation::{rotation_z, scaling, Transformation},
        tuples::{point, vector},
        util::RtcFl,
    };

    #[test]
    fn creating_a_sphere() {
        // Since id's are given by the global static function in utils.rs, and tests apparently run
        // async, there is no guarantee that the id's will be 1, 2, 3. Instead, we ensure that they are
        // unique and sequential.
        let s1 = Sphere::default(); // .id = n
        let s2 = Sphere::default(); // .id = n + 1
        let s3 = Sphere::default(); // .id = n + 2

        let n = s1.id;

        assert_eq!(s1.id, n);
        assert_eq!(s2.id, n + 1);
        assert_eq!(s3.id, n + 2);
    }

    #[test]
    fn spheres_default_transform_is_identity_matrix() {
        let s = Sphere::default();
        let m = Matrix4::identity();

        assert_eq!(s.transform, m);
    }

    #[test]
    fn changing_spheres_transform() {
        let mut s = Sphere::default();
        let t = Transformation::new().translation(2.0, 3.0, 4.0);

        s.transform = t.get();

        assert_eq!(s.transform, t.get());
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();
        s.transform = Transformation::new().scaling(2.0, 2.0, 2.0).get();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();
        s.transform = Transformation::new().translation(5.0, 0.0, 0.0).get();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::default();
        let n = s.normal_at(point(1.0, 0.0, 0.0));

        assert_eq!(vector(1.0, 0.0, 0.0), n);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::default();
        let n = s.normal_at(point(0.0, 1.0, 0.0));

        assert_eq!(vector(0.0, 1.0, 0.0), n);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::default();
        let n = s.normal_at(point(0.0, 0.0, 1.0));

        assert_eq!(vector(0.0, 0.0, 1.0), n);
    }

    #[test]
    fn normal_on_a_sphere_at_a_non_axial_point() {
        let s = Sphere::default();
        let sqrt3over3 = (3.0 as RtcFl).sqrt() / 3.0;
        let n = s.normal_at(point(sqrt3over3, sqrt3over3, sqrt3over3));

        assert_eq!(vector(sqrt3over3, sqrt3over3, sqrt3over3), n);
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::default();
        let sqrt3over3 = (3.0 as RtcFl).sqrt() / 3.0;
        let n = s.normal_at(point(sqrt3over3, sqrt3over3, sqrt3over3));

        assert_eq!(n.normalize(), n);
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let mut s = Sphere::default();
        s.transform = Transformation::new().translation(0.0, 1.0, 0.0).get();

        let n = s.normal_at(point(0.0, 1.70711, -0.70711));

        assert_eq!(vector(0.0, 0.70711, -0.70711), n);
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut s = Sphere::default();
        s.transform = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        let sqrt2over2 = (2.0 as RtcFl).sqrt() / 2.0;

        let n = s.normal_at(point(0.0, sqrt2over2, -sqrt2over2));

        //assert_eq!(vector(0.0, 0.97014, -0.24254), n);
        assert!(vector(0.0, 0.97014, -0.24254).eq(&n));
    }

    #[test]
    fn a_sphere_has_default_material() {
        let s = Sphere::default();
        let m = s.material;

        assert_eq!(m, Material::default());
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let mut s = Sphere::default();
        let mut m = Material::default();
        m.shininess = 900.0;
        s.material = m;

        assert_eq!(s.material.shininess, 900.0);
    }
}
