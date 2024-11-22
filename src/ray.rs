use crate::tuples::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {

    pub fn new(o: Tuple, d: Tuple) -> Self {
        if !o.is_point() || !d.is_vector() {
            panic!("Origin must be a point. Direction must be a vector.");
        }

        Self {
            origin: o,
            direction: d,
        }

    }
}