use std::ops;

const EPSILON: f32 = 0.00001_f32;

fn equal(a: f32, b: f32) -> bool {
    // Compare two f32 values for equality within the constant EPSILON
    (a - b).abs() <= f32::EPSILON // Try built-in EPSILON value, but change to my constant if it doesn't work out.
}

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let m = self.mag();
        Self::new(self.x / m, self.y / m, self.z / m, self.w / m)
    }

    pub fn dot(&self, other: Self) -> f32 {
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
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
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

impl ops::Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl ops::Div<f32> for Tuple {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        Self::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
    }
}

#[cfg(test)]
mod tests;
