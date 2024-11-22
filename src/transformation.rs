use crate::matrix::Matrix4x4;
use crate::util::RtcFl;

#[derive(Clone, Copy)]
pub struct Transformation {
    t: Matrix4x4,
}

impl Transformation {

    pub fn new() -> Self {
        Self {
            t: Matrix4x4::identity(),
        }
    }

    pub fn get(&self) -> Matrix4x4 {
        self.t
    }
    
    pub fn translation(mut self, x: RtcFl, y: RtcFl, z: RtcFl) -> Self {
        let mut m = Matrix4x4::identity();
        m[(0, 3)] = x;
        m[(1, 3)] = y;
        m[(2, 3)] = z;
        
        self.t = m * self.t;

        self
    }

    pub fn scaling(mut self, x: RtcFl, y: RtcFl, z: RtcFl) -> Self {
        let mut m = Matrix4x4::identity();
        m[(0, 0)] = x;
        m[(1, 1)] = y;
        m[(2, 2)] = z;

        self.t = m * self.t;
        
        self
    }

    pub fn rotation_x(mut self, r: RtcFl) -> Self {
        let mut m = Matrix4x4::identity();
        m[(1, 1)] = r.cos();
        m[(1, 2)] = -r.sin();
        m[(2, 1)] = r.sin();
        m[(2, 2)] = r.cos();
        
        self.t = m * self.t;
        
        self
    }

    pub fn rotation_y(mut self, r: RtcFl) -> Self {
        let mut m = Matrix4x4::identity();
        m[(0, 0)] = r.cos();
        m[(0, 2)] = r.sin();
        m[(2, 0)] = -r.sin();
        m[(2, 2)] = r.cos();

        self.t = m * self.t;
        
        self
    }

    pub fn rotation_z(mut self, r: RtcFl) -> Self {
        let mut m = Matrix4x4::identity();
        m[(0, 0)] = r.cos();
        m[(0, 1)] = -r.sin();
        m[(1, 0)] = r.sin();
        m[(1, 1)] = r.cos();

        self.t = m * self.t;
        
        self
    }

    pub fn shearing(mut self, xy: RtcFl, xz: RtcFl, yx: RtcFl, yz: RtcFl, zx: RtcFl, zy: RtcFl) -> Self {
        let mut m = Matrix4x4::identity();
        m[(0, 1)] = xy;
        m[(0, 2)] = xz;
        m[(1, 0)] = yx;
        m[(1, 2)] = yz;
        m[(2, 0)] = zx;
        m[(2, 1)] = zy;

        self.t = m * self.t;
        
        self
    }
}
/* 
// Shortcut functions
pub fn tr(x: RtcFl, y: RtcFl, z: RtcFl) -> Matrix4x4 {
    Transformation::translation(x, y, z)
}

pub fn sc(x: RtcFl, y: RtcFl, z: RtcFl) -> Matrix4x4 {
    Transformation::scaling(x, y, z)
}

pub fn rx(r: RtcFl) -> Matrix4x4 {
    Transformation::rotation_x(r)
}

pub fn ry(r: RtcFl) -> Matrix4x4 {
    Transformation::rotation_y(r)
}

pub fn rz(r: RtcFl) -> Matrix4x4 {
    Transformation::rotation_z(r)
}

pub fn sh(xy: RtcFl, xz: RtcFl, yx: RtcFl, yz: RtcFl, zx: RtcFl, zy: RtcFl) -> Matrix4x4 {
    Transformation::shearing(xy, xz, yx, yz, zx, zy)
}
    */