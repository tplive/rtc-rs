use crate::{
    intersections::{Intersection, Intersections},
    material::Material,
    matrix::Matrix4,
    ray::Ray,
    shape::{next_shape_id, Shape},
    tuples::{point, vector, Tuple},
    util::EPSILON,
};

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    pub id: usize,
    pub transform: Matrix4,
    pub material: Material,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            id: next_shape_id(),
            transform: Matrix4::identity(),
            material: Material::default(),
        }
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.material == other.material && self.transform == other.transform
    }
}

impl Plane {
    pub fn new(transform: Matrix4, material: Material) -> Self {
        Self {
            id: next_shape_id(),
            transform,
            material,
        }
    }
}

impl Shape for Plane {
    fn intersect<'s>(&'s self, ray: &Ray) -> Vec<Intersection<'s>> {
        if ray.direction.y.abs() < EPSILON {
            vec![]
        } else {
            let t = -ray.origin.y / ray.direction.y;
            vec![Intersection::new(t, self)]
        }
    }

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        vector(0.0, 1.0, 0.0)
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
    use crate::{
        ray::Ray,
        shape::Shape,
        tuples::{point, vector},
    };

    use super::Plane;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::default();

        let n1 = p.normal_at(point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(point(-5.0, 0.0, -150.0));

        assert!(n1 == vector(0.0, 1.0, 0.0));
        assert!(n2 == vector(0.0, 1.0, 0.0));
        assert!(n3 == vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let p = Plane::default();
        let r = Ray::new(&point(0.0, 10.0, 0.0), &vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = Plane::default();
        let r = Ray::new(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = Plane::default();
        let r = Ray::new(&point(0.0, 1.0, 0.0), &vector(0.0, -1.0, 0.0));
        let xs = p.intersect(&r);
        assert!(xs.iter().count() == 1);
        assert!(xs[0].t == 1.0);
        assert!(xs[0].shape.id() == p.id());
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let p = Plane::default();
        let r = Ray::new(&point(0.0, -1.0, 0.0), &vector(0.0, 1.0, 0.0));
        let xs = p.intersect(&r);
        assert!(xs.iter().count() == 1);
        assert!(xs[0].t == 1.0);
        assert!(xs[0].shape.id() == p.id());
    }
}
