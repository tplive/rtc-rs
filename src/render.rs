use indicatif::ProgressBar;

use crate::{camera::{ray_for_pixel, Camera}, canvas::Canvas, world::{color_at, World}};


pub fn render(camera: &Camera, world: World, bar: &ProgressBar) -> Canvas {

    let mut canvas = Canvas::new(camera.hsize, camera.vsize);

    for y in 0..camera.vsize -1 {
        for x in 0..camera.hsize -1 {
            bar.inc(1);
            let ray = ray_for_pixel(camera, x, y);
            let color = color_at(&world, ray);
            canvas.write_pixel(x, y, color);
        }
    }

    canvas
}