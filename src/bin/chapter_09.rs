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
    material::Material,
    matrix::view_transform,
    plane::Plane,
    render::render_parallel,
    sphere::Sphere,
    transformation::{scaling, translation},
    tuples::{point, vector},
    util::PI,
    world::World,
};

use sysinfo::{get_current_pid, System};

// Putting it together Chapter 9
fn main() {
    // Start timing the run:
    let now = Instant::now();
    println!("Rendering...");

    // Set up the scene
    let image_width = 1920;
    let aspect_ratio = (16.0_f32 / 9.0).round() as usize;
    let image_height = image_width / aspect_ratio;
    println!("Image size: {}x{}", image_width, image_height,);

    // Floor
    let floor = Plane {
        material: Material::new(Color::random(), None, 0.1, 1.0, 1.0, 1.0),
        ..Default::default()
    };

    // Middle sphere
    let mut middle = Sphere {
        transform: translation(-0.5, 1.0, 0.5),
        ..Default::default()
    };
    middle.material.color = Color::random();
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    // Right sphere
    let mut right = Sphere {
        transform: translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5),
        ..Default::default()
    };
    right.material.color = Color::random();
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    // Left sphere
    let mut left = Sphere {
        transform: translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33),
        ..Default::default()
    };
    left.material.color = Color::random();
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    // World
    let mut world = World::default();

    // Add plane
    world.add_object(floor);

    // Add objects
    world.add_objects(vec![left, middle, right]);

    world.light = vec![Light::point(point(-10.0, 10.0, -10.0), Color::white())];

    let mut camera = Camera::new(image_width, image_height, PI / 3.0);
    camera.transform = view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    );

    let bar = ProgressBar::new((image_width * image_width) as u64);
    bar.enable_steady_tick(Duration::from_millis(250));

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
    let path = "rendered/chapter_09.png";
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
