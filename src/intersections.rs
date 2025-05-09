use crate::{shape::Shape, util::{equal, RtcFl}};


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
    use crate::ray::Ray;
    use crate::tuples::{point, vector};
    use crate::world::tests::create_default_world_for_test;

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
}