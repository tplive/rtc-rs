use crate::color::Color;
use crate::computation::Computation;
use crate::intersections::{Intersection, Intersections};
use crate::light::{lighting, Light};
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::transformation::scaling;
use crate::tuples::{point, Tuple};

#[derive(Default)]
pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub light: Vec<Light>,
}

impl Clone for World {
    fn clone(&self) -> Self {
        Self {
            objects: self.objects.iter().map(|obj| obj.clone_boxed()).collect(),
            light: self.light.clone(),
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

    pub fn add_objects(&mut self, shapes: Vec<impl Shape + 'static>) {
        for s in shapes {
            self.add_object(s);
        }
    }

    pub fn shade_hit(&self, comps: Computation) -> Color {
        lighting(
            comps.shape.material(),
            comps.shape.as_ref(),
            &self.light[0],
            &comps.point,
            &comps.eyev,
            &comps.normalv,
            self.is_shadowed(&comps.over_point),
        )
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(ray);
        match intersections.hit() {
            Some(hit) => {
                let comps = Computation::new(hit, ray);
                self.shade_hit(comps)
            }
            None => Color::black(),
        }
    }

    pub fn is_shadowed(&self, point: &Tuple) -> bool {
        let v = self.light[0].position - *point;
        let distance = v.mag();
        let direction = v.normalize();

        let r = Ray::new(point, &direction);
        let intersections = self.intersect(&r);

        if let Some(hit) = intersections.hit() {
            // If the hit object is closer than the light source
            hit.t < distance
        } else {
            false
        }
    }
}

pub fn color_at(world: &World, ray: Ray) -> Color {
    let intersections = world.intersect(&ray);

    match intersections.hit() {
        Some(hit) => {
            let comps = Computation::new(hit, &ray);
            world.shade_hit(comps)
        }
        None => Color::black(),
    }
}

pub fn create_default_world_for_test() -> World {
    let light = Light::point(point(-10.0, 10.0, -10.0), Color::white());

    let mut s1_created = Sphere::default();
    s1_created.material.color = Color::new(0.8, 1.0, 0.6);
    s1_created.material.diffuse = 0.7;
    s1_created.material.specular = 0.2;

    let s2_created = Sphere {
        transform: scaling(0.5, 0.5, 0.5),
        ..Default::default()
    };

    let mut world = World::default();
    world.add_object(s1_created);
    world.add_object(s2_created);
    world.light.push(light);

    world
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::computation::Computation;
    use crate::intersections::Intersection;
    use crate::light::Light;
    use crate::material::Material;
    use crate::matrix::{view_transform, Matrix4, Operations};
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::transformation::{scaling, translation, Transformation};
    use crate::tuples::{point, vector};
    use crate::world::{create_default_world_for_test, World};

    #[test]
    fn creating_a_world() {
        let w = World::default();

        assert!(w.objects.is_empty());
        assert!(w.light.is_empty());
    }

    #[test]
    fn the_default_world() {
        let w = create_default_world_for_test();

        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::default();
        s2.transform = scaling(0.5, 0.5, 0.5);

        assert!(w.light.len() == 1);
        assert!(w.light[0].intensity == Color::new(1.0, 1.0, 1.0));
        assert!(w.light[0].position == point(-10.0, 10.0, -10.0));

        assert_eq!(w.objects.len(), 2);
        assert!(w
            .objects
            .iter()
            .any(|shape| shape.material() == &s1.material && shape.transform() == &s1.transform));
        assert!(w
            .objects
            .iter()
            .any(|shape| shape.material() == &s2.material && shape.transform() == &s2.transform));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = create_default_world_for_test();
        let r = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));

        let xs = w.intersect(&r);

        assert!(xs.data.len() == 4);
        assert!(xs.data[0].t == 4.0);
        assert!(xs.data[1].t == 4.5);
        assert!(xs.data[2].t == 5.5);
        assert!(xs.data[3].t == 6.0);
    }

    #[test]
    fn transformation_matrix_for_default_orientation() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);

        assert_eq!(t, Matrix4::identity());
    }

    #[test]
    fn view_transformation_matrix_looking_in_positive_z_direction() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);

        assert_eq!(t, Transformation::new().scaling(-1.0, 1.0, -1.0).get());
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);

        assert_eq!(t, Transformation::new().translation(0.0, 0.0, -8.0).get());
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);

        let t = view_transform(from, to, up);

        // Forward: Tuple { x: 0.35856858, y: -0.59761435, z: 0.71713716, w: 0.0 }

        #[rustfmt::skip]
        let m = Matrix4::new(
            -0.50709, 0.50709, 0.67612, -2.36643,
            0.76772, 0.60609, 0.12122, -2.82843,
            -0.35857, 0.59761, -0.71714, 0.00000,
            0.00000, 0.00000, 0.00000, 1.00000
        );

        assert!(m.equals(t));
    }

    #[test]
    fn no_shadow_when_nothing_colinear_with_point_and_light() {
        let w = create_default_world_for_test();

        let p = point(0.0, 10.0, 0.0);

        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = create_default_world_for_test();

        let p = point(10.0, -10.0, 10.0);

        assert_eq!(w.is_shadowed(&p), true);
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = create_default_world_for_test();

        let p = point(-20.0, 20.0, -20.0);

        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = create_default_world_for_test();

        let p = point(-2.0, 2.0, -2.0);

        assert_eq!(w.is_shadowed(&p), false);
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::default();
        w.light
            .push(Light::point(point(0.0, 0.0, -10.0), Color::white()));
        let s1 = Sphere::default();
        let s2 = Sphere::new(translation(0.0, 0.0, 10.0), Material::default());

        w.add_objects(vec![s1.clone(), s2.clone()]);

        let r = Ray::new(&point(0.0, 0.0, 5.0), &vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, &s2);
        let comps = Computation::new(i, &r);
        let c = w.shade_hit(comps);

        assert!(c == Color::new(0.1, 0.1, 0.1));
    }
}
