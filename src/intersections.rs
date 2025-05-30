use crate::{
    shape::Shape,
    util::{equal, RtcFl},
};

#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: RtcFl,
    pub shape: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: RtcFl, shape: &'a dyn Shape) -> Self {
        Self { t, shape }
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        equal(self.t, other.t) && self.shape.id() == other.shape.id()
    }
}

pub struct Intersections<'a> {
    pub(crate) data: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new(mut data: Vec<Intersection<'a>>) -> Self {
        data.sort_unstable_by(|a, b| {
            a.t.partial_cmp(&b.t)
                .expect("Unable to sort intersections!")
        });
        Self { data }
    }

    pub fn hit(&self) -> Option<Intersection<'a>> {
        for n in self.data.iter() {
            if n.t >= 0.0 {
                return Some(*n);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::computation::Computation;
    use crate::intersections::Intersection;
    use crate::light::Light;
    use crate::material::Material;
    use crate::ray::Ray;
    use crate::shape::Sphere;
    use crate::transformation::{scaling, translation};
    use crate::tuples::{point, vector};
    use crate::util::EPSILON;
    use crate::world::{create_default_world_for_test, World};

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

        let ray = Ray::new(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));

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

    #[test]
    fn color_with_intersection_behind_ray() {
        let light = Light::point(point(-10.0, 10.0, -10.0), Color::white());

        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        s1.material.ambient = 1.0;

        let mut s2 = Sphere::default();
        s2.transform = scaling(0.5, 0.5, 0.5);
        s2.material.ambient = 1.0;

        let mut w = World::default();
        w.add_object(s1);
        w.add_object(s2);
        w.light.push(light);

        let r = Ray::new(&point(0.0, 0.0, 0.75), &vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r);

        assert_eq!(c, w.objects[1].material().color);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let shape = Sphere::new(translation(0.0, 0.0, 1.0), Material::default());
        let i = Intersection::new(5.0, &shape);
        let comps = Computation::new(i, &r);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
