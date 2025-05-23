use crate::{matrix::Matrix4, util::RtcFl};


pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: RtcFl,
    pub transform: Matrix4,
}

impl Camera {
    pub fn new(hsize: &usize, vsize: &usize, fov: &RtcFl ) -> Self {

        let transform = Matrix4::identity();
        Self {
            hsize: *hsize,
            vsize: *vsize,
            fov: *fov,
            transform,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{camera::Camera, matrix::Matrix4, util::PI};


    #[test]
    fn constructing_camera() {

        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let c = Camera::new(&hsize, &vsize, &field_of_view);

        assert!(c.hsize == 160);
        assert!(c.vsize == 120);
        assert!(c.fov == PI / 2.0);
        assert!(c.transform == Matrix4::identity());
    }
}