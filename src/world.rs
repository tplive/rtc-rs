use crate::color::Color;
use crate::computation::Computation;
use crate::light::{lighting, Light};
use crate::ray::Ray;
use crate::intersections::{Intersections, Intersection};
use crate::shape::Shape;

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
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::computation::Computation;
    use crate::intersections::Intersection;
    use crate::light::Light;
    use crate::ray::Ray;
    use crate::shape::Sphere;
    use crate::transformation::scaling;
    use crate::tuples::{point, vector};
    use crate::world::World;

    #[test]
    fn creating_a_world() {
        let w = World::default();

        assert!(w.objects.is_empty());
        assert!(w.light.is_empty());
    }

    pub fn create_default_world_for_test() -> World {
        let light = Light::point(point(-10.0, 10.0, -10.0), Color::white());

        let mut s1_created = Sphere::default();
        s1_created.material.color = Color::new(0.8, 1.0, 0.6);
        s1_created.material.diffuse = 0.7;
        s1_created.material.specular = 0.2;

        let mut s2_created = Sphere::default();
        s2_created.transform = scaling(0.5, 0.5, 0.5);

        let mut world = World::default();
        world.add_object(s1_created);
        world.add_object(s2_created);
        world.light.push(light);

        world
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
    fn shading_an_intersection() {
        let world = create_default_world_for_test();
        let ray = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));

        // The first object in the world
        let shape = world.objects[0].as_ref();
        let i = Intersection::new(4.0, shape);
        let comps = Computation::new(i, &ray);

        let c = world.shade_hit(comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut world = create_default_world_for_test();
        world.light[0] = Light::point(point(0.0, 0.25, 0.0), Color::white());

        let ray = Ray::new(&point(0.0, 0.25, 0.0), &vector(0.0, 0.0, 1.0));

        // The second object in the world
        let shape = world.objects[1].as_ref();
        let i = Intersection::new(0.5, shape);
        let comps = Computation::new(i, &ray);

        let c = world.shade_hit(comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = create_default_world_for_test();
        let r = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r);

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = create_default_world_for_test();
        let r = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    //#[test]
    // fn color_with_intersection_behind_ray() {
    //     let mut w = create_default_world_for_test();

    //     let inner = w.objects[0].material();
    //     inner.ambient = 1.0;

    //     let r = Ray::new(&point(0.0, 0.0, 0.75), &vector(0.0, 0.0, -1.0));
    //     let c = w.color_at(&r);

    //     assert_eq!(c, inner.color);
    // }
}
