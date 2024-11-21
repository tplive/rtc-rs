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

    pub fn rotation_x(rad: RtcFl) -> Matrix4x4 {
        let mut m = Matrix4x4::identity();
        m[(1, 1)] = rad.cos();
        m[(1, 2)] = -rad.sin();
        m[(2, 1)] = rad.sin();
        m[(2, 2)] = rad.cos();
        
        m
    }

    pub fn rotation_y(r: RtcFl) -> Matrix4x4 {
        let mut m = Matrix4x4::identity();
        m[(0, 0)] = r.cos();
        m[(0, 2)] = r.sin();
        m[(2, 0)] = -r.sin();
        m[(2, 2)] = r.cos();
        
        m
    }

    pub fn rotation_z(r: RtcFl) -> Matrix4x4 {
        let mut m = Matrix4x4::identity();
        m[(0, 0)] = r.cos();
        m[(0, 1)] = -r.sin();
        m[(1, 0)] = r.sin();
        m[(1, 1)] = r.cos();
        
        m
    }
    
}
