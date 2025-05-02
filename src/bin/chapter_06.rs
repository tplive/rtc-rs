extern crate rtc_rs as rtc;

use std::{
    fs::File,
    io::{BufWriter, Write},
    time::{Duration, Instant},
};

use rtc::{
    canvas::Canvas,
    color::Color,
    light::{lighting, Light},
    ray::Ray,
    shape::{Intersectable, Intersections, NormalAt, Sphere},
    tuples::point,
    util::RtcFl,
};

use indicatif::ProgressBar;

// Putting it together Chapter 6
fn main() {
    let now = Instant::now();
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z: RtcFl = 10.0;
    let wall_size: RtcFl = 7.0;
    let canvas_pixels = 256;
    let pixel_size: RtcFl = wall_size / canvas_pixels as RtcFl;
    let half: RtcFl = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = Sphere::default();
    shape.material.color = Color::new(1.0, 0.2, 1.0);

    let light = Light::point(point(-10.0, 10.0, -10.0), Color::white());

    let bar = ProgressBar::new((canvas_pixels * canvas_pixels) as u64);
    bar.enable_steady_tick(Duration::from_millis(250));

    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as RtcFl;
        for x in 0..canvas_pixels - 1 {
            bar.inc(1);
            let world_x = -half + pixel_size * x as RtcFl;
            let position = (point(world_x, world_y, wall_z) - ray_origin).normalize();
            let r = Ray::new(&ray_origin, &position);
            let xs = Intersections::new(shape.intersect(&r));

            match xs.hit() {
                Some(the_hit) => {
                    let the_hit_normal = &the_hit.shape.normal_at(position);
                    let color = lighting(
                        &shape.material,
                        &light,
                        &r.position(the_hit.t),
                        &-r.direction,
                        the_hit_normal,
                    );
                    canvas.write_pixel(x, y, color);
                }

                None => {
                    canvas.write_pixel(x, y, Color::black());
                }
            }
        }
    }

    bar.finish();

    let elapsed = now.elapsed();
    println!("Elapsed time for rendering: {:.2?}", elapsed);

    // Write to PPM file
    let mut ppm_file = File::create("chapter_06.ppm").expect("Unable to create file.");

    ppm_file
        .write_all(canvas.to_ppm().as_bytes())
        .expect("Unable to write file.");

    // Write to PNG file
    let png_file = File::create("rendered/chapter_06.png").expect("Unable to create file.");
    let w = &mut BufWriter::new(png_file);
    let mut encoder = png::Encoder::new(w, canvas.width as u32, canvas.height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&canvas.to_png()).unwrap();
}
