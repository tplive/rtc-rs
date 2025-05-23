use crate::matrix::Matrix4;
use crate::util::RtcFl;

#[derive(Debug, Clone, Copy)]
pub struct Transformation {
    t: Matrix4,
}

impl Transformation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self) -> Matrix4 {
        self.t
    }

    pub fn translation(mut self, x: RtcFl, y: RtcFl, z: RtcFl) -> Self {
        let mut m = Matrix4::identity();
        m[(0, 3)] = x;
        m[(1, 3)] = y;
        m[(2, 3)] = z;

        self.t = m * self.t;

        self
    }

    pub fn scaling(mut self, x: RtcFl, y: RtcFl, z: RtcFl) -> Self {
        let mut m = Matrix4::identity();
        m[(0, 0)] = x;
        m[(1, 1)] = y;
        m[(2, 2)] = z;

        self.t = m * self.t;

        self
    }

    pub fn rotation_x(mut self, r: RtcFl) -> Self {
        let mut m = Matrix4::identity();
        m[(1, 1)] = r.cos();
        m[(1, 2)] = -r.sin();
        m[(2, 1)] = r.sin();
        m[(2, 2)] = r.cos();

        self.t = m * self.t;

        self
    }

    pub fn rotation_y(mut self, r: RtcFl) -> Self {
        let mut m = Matrix4::identity();
        m[(0, 0)] = r.cos();
        m[(0, 2)] = r.sin();
        m[(2, 0)] = -r.sin();
        m[(2, 2)] = r.cos();

        self.t = m * self.t;

        self
    }

    pub fn rotation_z(mut self, r: RtcFl) -> Self {
        let mut m = Matrix4::identity();
        m[(0, 0)] = r.cos();
        m[(0, 1)] = -r.sin();
        m[(1, 0)] = r.sin();
        m[(1, 1)] = r.cos();

        self.t = m * self.t;

        self
    }

    pub fn shearing(
        mut self,
        xy: RtcFl,
        xz: RtcFl,
        yx: RtcFl,
        yz: RtcFl,
        zx: RtcFl,
        zy: RtcFl,
    ) -> Self {
        let mut m = Matrix4::identity();
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

impl Default for Transformation {
    fn default() -> Self {
        Self {
            t: Matrix4::identity(),
        }
    }
}

pub fn translation(x: RtcFl, y: RtcFl, z: RtcFl) -> Matrix4 {
    Transformation::new().translation(x, y, z).get()
}

pub fn scaling(x: RtcFl, y: RtcFl, z: RtcFl) -> Matrix4 {
    Transformation::new().scaling(x, y, z).get()
}

pub fn rotation_x(r: RtcFl) -> Matrix4 {
    Transformation::new().rotation_x(r).get()
}

pub fn rotation_y(r: RtcFl) -> Matrix4 {
    Transformation::new().rotation_y(r).get()
}

pub fn rotation_z(r: RtcFl) -> Matrix4 {
    Transformation::new().rotation_z(r).get()
}

pub fn shearing(xy: RtcFl, xz: RtcFl, yx: RtcFl, yz: RtcFl, zx: RtcFl, zy: RtcFl) -> Matrix4 {
    Transformation::new().shearing(xy, xz, yx, yz, zx, zy).get()
}
