use std::ops;

use crate::util::{equal, RtcFl};

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: RtcFl,
    pub y: RtcFl,
    pub z: RtcFl,
    pub w: RtcFl,
}

impl Tuple {
    pub fn new(x: RtcFl, y: RtcFl, z: RtcFl, w: RtcFl) -> Self {
        Self { x, y, z, w }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn mag(&self) -> RtcFl {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.mag();
        Self::new(self.x / m, self.y / m, self.z / m, self.w / m)
    }

    pub fn dot(&self, other: Self) -> RtcFl {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
            0.0,
        )
    }

    pub fn reflect(self, normal: Tuple) -> Self {
        self - normal * 2.0 * self.dot(normal)
    }
}

pub fn point(x: RtcFl, y: RtcFl, z: RtcFl) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: RtcFl, y: RtcFl, z: RtcFl) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        equal(self.x, other.x)
            && equal(self.y, other.y)
            && equal(self.z, other.z)
            && equal(self.w, other.w)
    }
}

impl ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl ops::Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl ops::Mul<RtcFl> for Tuple {
    type Output = Self;

    fn mul(self, scalar: RtcFl) -> Self::Output {
        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl ops::Div<RtcFl> for Tuple {
    type Output = Self;

    fn div(self, scalar: RtcFl) -> Self::Output {
        Self::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
    }
}
