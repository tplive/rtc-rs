extern crate rtc_rs as rtc;

use std::{fs::File, io::Write, time::Instant};

use rtc::{
    canvas::Canvas,
    color::Color,
    ray::Ray,
    shape::{Intersectable, Intersections, Sphere},
    tuples::point,
    util::RtcFl,
};

// Putting it together Chapter 4
fn main() {
    let now = Instant::now();
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z: RtcFl = 10.0;
    let wall_size: RtcFl = 7.0;
    let canvas_pixels = 400;
    let pixel_size: RtcFl = wall_size / canvas_pixels as RtcFl;
    let half: RtcFl = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.5, 0.5);
    let shape = Sphere::new();

    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as RtcFl;
        for x in 0..canvas_pixels - 1 {
            let world_x = -half + pixel_size * x as RtcFl;
            let position = point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = Intersections::new(shape.intersect(r));

            if xs.hit() != None {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed time for rendering: {:.2?}", elapsed);

    // Write to file
    let mut file = File::create("chapter_05.ppm").expect("Unable to create file.");

    file.write_all(&canvas.to_ppm().as_bytes())
        .expect("Unable to write file.");
}