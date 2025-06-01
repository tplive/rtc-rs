extern crate rtc_rs as rtc;

use std::{
    fs::File,
    io::BufWriter,
    time::{Duration, Instant},
};

use indicatif::ProgressBar;
use rtc::{
    camera::Camera,
    color::Color,
    light::Light,
    matrix::view_transform,
    render::{render, render_parallel},
    sphere::Sphere,
    transformation::{rotation_y, rotation_z, scaling, translation},
    tuples::{point, vector},
    util::PI,
    world::World,
};

use sysinfo::{get_current_pid, System};

// Putting it together Chapter 8
fn main() {
    // Start timing the run:
    let now = Instant::now();
    println!("Rendering...");

    // Set up the scene
    let canvas_pixels = 2048;
    println!("Image size: {}x{}", canvas_pixels, canvas_pixels);

    // Floor
    let mut floor = Sphere::default();
    floor.transform = scaling(10.0, 0.01, 10.0);
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    // Left wall
    let mut left_wall = Sphere::default();
    left_wall.transform = translation(0.0, 0.0, 5.0)
        * rotation_y(-PI / 4.0)
        * rotation_z(PI / 2.0)
        * scaling(10.0, 0.01, 10.0);
    left_wall.material = floor.material;

    // Right wall
    let mut right_wall = Sphere::default();
    right_wall.transform = translation(0.0, 0.0, 5.0)
        * rotation_y(PI / 4.0)
        * rotation_z(PI / 2.0)
        * scaling(10.0, 0.01, 10.0);
    right_wall.material = floor.material;

    // Middle sphere
    let mut middle = Sphere::default();
    middle.transform = translation(-0.5, 1.0, 0.5);
    middle.material.color = Color::random();
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    // Right sphere
    let mut right = Sphere::default();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material.color = Color::random();
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    // Left sphere
    let mut left = Sphere::default();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material.color = Color::random();
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    // World
    let mut world = World::default();
    // Add objects
    world.add_objects(vec![floor, left_wall, right_wall, left, middle, right]);

    world.light = vec![Light::point(point(-10.0, 10.0, -10.0), Color::white())];

    let mut camera = Camera::new(canvas_pixels, canvas_pixels, PI / 3.0);
    camera.transform = view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    );

    let bar = ProgressBar::new((canvas_pixels * canvas_pixels) as u64);
    bar.enable_steady_tick(Duration::from_millis(250));

    // If you insist, you can also run this non-parallel:
    // let canvas = render(&camera, world, &bar);
    let canvas = render_parallel(&camera, &world, &bar, false);
    bar.finish();

    let elapsed = now.elapsed();
    println!("Elapsed time for rendering: {:.2?}", elapsed);
    

    // Report memory usage
    let mut system = System::new_all();
    system.refresh_all();
    let process = system.process(get_current_pid().unwrap()).unwrap();
    println!("Memory usage: {:.2} MB", process.memory() as f64 / 1024.0);

    let now = Instant::now();

    // Write to PNG file
    let path = "rendered/chapter_08.png";
    println!("Writing to file '{}'...", &path);
    let png_file = File::create(path).expect("Unable to create file.");
    let w = &mut BufWriter::new(png_file);
    let mut encoder = png::Encoder::new(w, canvas.width as u32, canvas.height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&canvas.to_png()).unwrap();
    
    let elapsed = now.elapsed();
    println!("Elapsed time for saving file: {:.2?}", elapsed);
    
}
