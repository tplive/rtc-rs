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

#[cfg(test)]
mod tests {
    use crate::{
        transformation::{
            rotation_x, rotation_y, rotation_z, scaling, shearing, translation, Transformation,
        },
        tuples::{point, vector},
    };
    use std::f32::consts::PI;

    #[test]
    fn multiplying_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);

        let p = point(-3.0, 4.0, 5.0);

        //println!("{}", transform);
        assert_eq!(point(2.0, 1.0, 7.0), transform * p);
    }
    #[test]
    fn multiplying_by_inverse_of_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0).try_inverse().unwrap();
        let p = point(-3.0, 4.0, 5.0);

        assert_eq!(point(-8.0, 7.0, 3.0), transform * p);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Transformation::new()
            .translation(5.0, -3.0, 2.0)
            .get()
            .try_inverse()
            .unwrap();
        let v = vector(-3.0, 4.0, 5.0);

        assert_eq!(vector(-3.0, 4.0, 5.0), transform * v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = Transformation::new().scaling(2.0, 3.0, 4.0).get();
        let p = point(-4.0, 6.0, 8.0);
        //println!("{}", transform);

        assert_eq!(point(-8.0, 18.0, 32.0), transform * p);
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = Transformation::new().scaling(2.0, 3.0, 4.0).get();
        let v = vector(-4.0, 6.0, 8.0);
        //println!("{}", transform);

        assert_eq!(vector(-8.0, 18.0, 32.0), transform * v);
    }

    #[test]
    fn multiply_by_inverse_of_scaling_matrix() {
        let transform = Transformation::new()
            .scaling(2.0, 3.0, 4.0)
            .get()
            .try_inverse()
            .unwrap();
        let v = vector(-4.0, 6.0, 8.0);
        //println!("{}", transform);

        assert_eq!(vector(-2.0, 2.0, 2.0), transform * v);
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        //println!("{}", transform);

        assert_eq!(point(-2.0, 3.0, 4.0), transform * p);
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        let sqrt2div2 = 2.0_f32.sqrt() / 2.0;

        assert_eq!(half_quarter * p, point(0.0, sqrt2div2, sqrt2div2));
        assert_eq!(full_quarter * p, point(0.0, 0.0, 1.0,));
    }

    #[test]
    fn inverse_of_x_rotation_goes_opposite_direction() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = Transformation::new()
            .rotation_x(PI / 4.0)
            .get()
            .try_inverse()
            .unwrap();

        let sqrt2div2 = 2.0_f32.sqrt() / 2.0;

        assert_eq!(half_quarter * p, point(0.0, sqrt2div2, -sqrt2div2));
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        let sqrt2div2 = 2.0_f32.sqrt() / 2.0;

        assert_eq!(half_quarter * p, point(sqrt2div2, 0.0, sqrt2div2));
        assert_eq!(full_quarter * p, point(1.0, 0.0, 0.0,));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        let sqrt2div2 = 2.0_f32.sqrt() / 2.0;

        assert_eq!(half_quarter * p, point(-sqrt2div2, sqrt2div2, 0.0));
        assert_eq!(full_quarter * p, point(-1.0, 0.0, 0.0,));
    }

    #[test]
    fn shearing_x_in_proportion_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_x_in_proportion_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_y_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_y_in_proportion_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_z_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(2.0, 3.0, 6.0));
    }
    #[test]
    fn shearing_z_in_proportion_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = point(1.0, 0.0, 1.0);
        let a = Transformation::new().rotation_x(PI / 2.0).get();
        let b = Transformation::new().scaling(5.0, 5.0, 5.0).get();
        let c = Transformation::new().translation(10.0, 5.0, 7.0).get();

        // Apply rotation first
        let p2 = a * p;
        // Apply scaling
        let p3 = b * p2;
        // Apply translation
        let p4 = c * p3;

        assert_eq!(p2, point(1.0, -1.0, 0.0));
        assert_eq!(p3, point(5.0, -5.0, 0.0));
        assert_eq!(p4, point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformation_must_be_applied_in_reverse_order() {
        let p = point(1.0, 0.0, 1.0);
        let a = Transformation::new().rotation_x(PI / 2.0).get();
        let b = Transformation::new().scaling(5.0, 5.0, 5.0).get();
        let c = Transformation::new().translation(10.0, 5.0, 7.0).get();

        let t = c * b * a;

        assert_eq!(t * p, point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chaining_transformations_in_reverse_sequence() {
        let p = point(1.0, 0.0, 1.0);
        let transform = Transformation::new()
            .rotation_x(PI / 2.0)
            .scaling(5.0, 5.0, 5.0)
            .translation(10.0, 5.0, 7.0)
            .get();

        assert_eq!(transform * p, point(15.0, 0.0, 7.0));
    }

    #[test]
    fn multiply_shortcut_translations_right_order() {
        let p = point(1.0, 0.0, 1.0);
        let transform = translation(10.0, 5.0, 7.0) * scaling(5.0, 5.0, 5.0) * rotation_x(PI / 2.0);

        assert_eq!(transform * p, point(15.0, 0.0, 7.0));
    }
}
