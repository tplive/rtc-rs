use crate::{
    matrix::Matrix4x4, tuples::Tuple, util::RtcFl
};

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

    pub fn position(&self, t: RtcFl) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: Matrix4x4) -> Self {
        Ray {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}


