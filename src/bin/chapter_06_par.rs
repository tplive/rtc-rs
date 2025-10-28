extern crate rtc_rs as rtc;

use std::{
    fs::File,
    io::BufWriter,
    sync::mpsc,
    thread::{self, available_parallelism},
    time::{Duration, Instant},
};

use rtc::{
    canvas::Canvas,
    color::Color,
    light::{lighting, Light},
    ray::Ray,
    shape::Shape,
    sphere::Sphere,
    intersections::Intersections,
    tuples::point,
    util::RtcFl,
};

use indicatif::ProgressBar;
use sysinfo::{get_current_pid, System};

// Putting it together Chapter 6
fn main() {
    // Start timing the run:
    let now = Instant::now();
    println!("Rendering...");

    // Set up the scene
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z: RtcFl = 10.0;
    let wall_size: RtcFl = 7.0;
    let canvas_pixels = 2048;
    println!("Image size: {}x{}", canvas_pixels, canvas_pixels);

    let pixel_size: RtcFl = wall_size / canvas_pixels as RtcFl;
    let half: RtcFl = wall_size / 2.0;

    // Create a canvas and a shape, configure the shape
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = Sphere::default();
    shape.material.color = Color::new(1.0, 0.2, 1.0);
    shape.material.ambient = 0.1;
    shape.material.diffuse = 0.9;
    shape.material.specular = 0.2;
    shape.material.shininess = 200.0;

    // Set up the light source
    let light = Light::point(point(-10.0, 10.0, -10.0), Color::white());

    // Add a progress bar
    let bar = ProgressBar::new((canvas_pixels * canvas_pixels) as u64);
    bar.enable_steady_tick(Duration::from_millis(250));

    // Initialize parallelism
    let (tx, rx) = mpsc::channel();
    let num_threads = available_parallelism().map(|n| n.get()).unwrap_or(1);
    println!("Number of threads: {}", num_threads);

    // Divide the work into chunks
    let mut pairs = Vec::new();
    for y in 0..canvas_pixels {
        for x in 0..canvas_pixels {
            pairs.push((x, y));
        }
    }

    let chunk_size = pairs.len() / num_threads;
    let chunks: Vec<_> = pairs.chunks(chunk_size).collect();

    // Run the threads
    for chunk in chunks {
        let tx = tx.clone();
        let chunk = chunk.to_vec();
        let cloned_shape = shape.clone();

        thread::spawn(move || {
            for (x, y) in chunk {
                // Calculate the world coordinates for the pixel
                let world_y = half - pixel_size * y as RtcFl;
                let world_x = -half + pixel_size * x as RtcFl;
                let position = (point(world_x, world_y, wall_z) - ray_origin).normalize();
                let r = Ray::new(&ray_origin, &position);
                let xs = Intersections::new(cloned_shape.intersect(&r).to_owned());

                // Determine the color of the pixel
                let color: Color = match xs.hit() {
                    Some(the_hit) => {
                        let hit_point = r.position(the_hit.t);
                        let normal_vector = &the_hit.shape.normal_at(hit_point);
                        let eye_vector = -r.direction;
                        
                        lighting(
                            &cloned_shape.material,
                            &light,
                            &hit_point,
                            &eye_vector,
                            normal_vector,
                            false,
                        )
                    }

                    // Otherwise, return a black pixel
                    None => Color::black()
                };

                // Send pixel coordinates and color to the main thread
                tx.send((x, y, color)).expect("Failed to send data.");
            }
        });
    }

    // Drop the transmitter
    drop(tx);

    // Receive pixels and write to canvas
    for (x, y, color) in rx {
        bar.inc(1);
        canvas.write_pixel(x, y, color);
    }

    // Finish the progress bar
    bar.finish();

    // Report memory usage
    let mut system = System::new_all();
    system.refresh_all();
    let process = system.process(get_current_pid().unwrap()).unwrap();
    println!("Memory usage: {:.2} MB", process.memory() as f64 / 1024.0);

    // Calculate and print elapsed time
    let elapsed = now.elapsed();

    println!("Elapsed time for rendering: {:.2?}", elapsed);

    // Write to PPM file
    // let mut ppm_file = File::create("chapter_06.ppm").expect("Unable to create file.");

    // ppm_file.write_all(&canvas.to_ppm().as_bytes())
    // .expect("Unable to write file.");

    // Write to PNG file
    let path = "rendered/chapter_06_par.png";
    println!("Writing to file '{}'...", &path);
    let png_file = File::create(path).expect("Unable to create file.");
    let w = &mut BufWriter::new(png_file);
    let mut encoder = png::Encoder::new(w, canvas.width as u32, canvas.height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&canvas.to_png()).unwrap();
}
