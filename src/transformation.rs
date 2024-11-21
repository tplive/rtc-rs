use crate::util::RtcFl;
use crate::matrix::Matrix4x4;

pub struct Transformation {}

impl Transformation {
    pub fn translation(x: RtcFl, y: RtcFl, z: RtcFl) -> Matrix4x4 {
        let mut m = Matrix4x4::identity();
        m[(0, 3)] = x;
        m[(1, 3)] = y;
        m[(2, 3)] = z;

        m
    }

    pub fn scaling(x: RtcFl, y: RtcFl, z: RtcFl) -> Matrix4x4 {
        let mut m = Matrix4x4::identity();
        m[(0, 0)] = x;
        m[(1, 1)] = y;
        m[(2, 2)] = z;

        m
    }
}
