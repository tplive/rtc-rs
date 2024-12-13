use crate::{
    matrix::Matrix4, tuples::Tuple, util::RtcFl
};


pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: &Tuple, direction: &Tuple) -> Self {
        if !origin.is_point() || !direction.is_vector() {
            panic!("Origin must be a point. Direction must be a vector.");
        }

        Self {
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn position(&self, t: RtcFl) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: Matrix4) -> Self {
        Ray {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}


