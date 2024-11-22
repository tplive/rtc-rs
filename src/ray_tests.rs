#[cfg(test)]
use crate::{
    ray::Ray,
    tuples::{point, vector},
};

#[test]
fn creating_and_querying_a_ray() {
    let origin = point(1.0, 2.0, 3.0);
    let direction = vector(4.0, 5.0, 6.0);

    let ray = Ray::new(origin, direction);

    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
}
