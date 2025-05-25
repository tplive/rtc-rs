use crate::{matrix::Matrix4, util::RtcFl};

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: RtcFl,
    pub transform: Matrix4,
    pub pixel_size: RtcFl,
    pub half_height: RtcFl,
    pub half_width: RtcFl,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: RtcFl) -> Self {
        let transform = Matrix4::identity();

        let half_view = (fov / 2.0).tan();
        let aspect = hsize as RtcFl / vsize as RtcFl;

        let half_width:RtcFl;
        let half_height:RtcFl;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / hsize as RtcFl;

        Self {
            hsize: hsize,
            vsize: vsize,
            fov: fov,
            transform,
            pixel_size,
            half_width,
            half_height,
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

        let c = Camera::new(hsize, vsize, field_of_view);

        assert!(c.hsize == 160);
        assert!(c.vsize == 120);
        assert!(c.fov == PI / 2.0);
        assert!(c.transform == Matrix4::identity());
    }

    #[test]
    fn pixel_size_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert_eq!(c.pixel_size, 0.01);
    }
    #[test]
    fn pixel_size_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert_eq!(c.pixel_size, 0.01);
    }
}
