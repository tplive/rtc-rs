use crate::{
    intersections::Intersection,
    ray::Ray,
    shape::Shape,
    tuples::Tuple,
    util::{RtcFl, SHADOW_EPSILON},
};

pub struct Computation {
    pub t: RtcFl,
    pub shape: Box<dyn Shape>,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

impl Computation {
    pub fn new(intersection: Intersection, ray: &Ray) -> Self {
        let t = intersection.t;
        // TODO: Computation could borrow the shape instead of owning it.
        // This could possibly be more performant, as it does not need
        // to clone the intersections. Would introduce lifetimes.
        let shape_obj: Box<dyn Shape> = intersection.shape.clone_boxed();
        let point = ray.position(t);
        let eyev = -ray.direction;
        let mut normalv = intersection.shape.normal_at(point);
        let inside = normalv.dot(eyev) < 0.0;
        if inside {
            normalv = -normalv;
        }
        let over_point = point + normalv * SHADOW_EPSILON;

        Self {
            t,
            shape: shape_obj,
            point,
            eyev,
            normalv,
            inside,
            over_point,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        computation::Computation,
        intersections::Intersection,
        ray::Ray,
        sphere::Sphere,
        tuples::{point, vector},
    };

    #[test]
    fn precompute_state_of_intersection() {
        let ray = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let i = Intersection::new(4.0, &sphere);

        let comps = Computation::new(i, &ray);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.shape.as_ref().id(), i.shape.id());
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_intersection_occurs_outside() {
        let ray = Ray::new(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);

        let comps = Computation::new(i, &ray);

        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_intersection_occurs_inside() {
        let ray = Ray::new(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(1.0, &shape);

        let comps = Computation::new(i, &ray);

        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }
}
