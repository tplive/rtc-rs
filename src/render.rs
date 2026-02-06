use std::{
    sync::mpsc,
    thread::{self, available_parallelism},
    time::{Duration, Instant},
};

use indicatif::ProgressBar;
use rand::seq::SliceRandom;

use crate::{
    camera::{ray_for_pixel, Camera},
    canvas::Canvas,
    world::{color_at, World},
};

pub fn render(camera: &Camera, world: World, bar: &ProgressBar) -> Canvas {
    let mut canvas = Canvas::new(camera.hsize, camera.vsize);

    for y in 0..camera.vsize - 1 {
        for x in 0..camera.hsize - 1 {
            bar.inc(1);
            let ray = ray_for_pixel(camera, x, y);
            let color = color_at(&world, ray);
            canvas.write_pixel(x, y, color);
        }
    }

    canvas
}

pub fn render_parallel(camera: &Camera, world: &World, bar: &ProgressBar, single: bool) -> Canvas {
    // Initialize parallelism
    let (tx, rx) = mpsc::channel();

    let num_threads: usize = if single {
        1
    } else {
        available_parallelism().map(|n| n.get()).unwrap_or(1)
    };

    println!("Number of threads: {}", num_threads);

    // Divide the work into chunks
    let mut pairs = Vec::new();
    for y in 0..camera.vsize - 1 {
        for x in 0..camera.hsize - 1 {
            pairs.push((x, y));
        }
    }

    let chunk_size = pairs.len() / num_threads;
    let chunks: Vec<_> = pairs.chunks(chunk_size).collect();

    // Run the threads
    for chunk in chunks {
        let tx = tx.clone();
        let chunk = chunk.to_vec();
        let world = world.clone();
        let camera = camera.clone();
        let bar = bar.clone();

        thread::spawn(move || {
            for (x, y) in chunk {
                bar.inc(1);
                let ray = ray_for_pixel(&camera, x, y);
                let color = color_at(&world, ray);
                tx.send((x, y, color)).expect("Failed to send pixel data.");
            }
        });
    }

    // Drop the transmitter
    drop(tx);

    // Init canvas
    let mut canvas = Canvas::new(camera.hsize, camera.vsize);

    for (x, y, color) in rx {
        canvas.write_pixel(x, y, color);
    }

    canvas
}

pub fn render_parallel_incremental<F>(
    camera: &Camera,
    world: &World,
    update_interval: Duration,
    mut on_update: F,
) -> Canvas
where
    F: FnMut(&Canvas),
{
    let (tx, rx) = mpsc::channel();

    let num_threads: usize = available_parallelism().map(|n| n.get()).unwrap_or(1);

    println!("Number of threads: {}", num_threads);

    let mut pairs = Vec::new();
    for y in 0..camera.vsize - 1 {
        for x in 0..camera.hsize - 1 {
            pairs.push((x, y));
        }
    }

    // Set to true to randomize pixels for effect
    // TODO: Implement GUI toggle
    if false {
        let mut rng = rand::rng();
        pairs.shuffle(&mut rng);
    }

    let chunk_size = pairs.len() / num_threads;
    let chunks: Vec<_> = pairs.chunks(chunk_size).collect();

    for chunk in chunks {
        let tx = tx.clone();
        let chunk = chunk.to_vec();
        let world = world.clone();
        let camera = camera.clone();

        thread::spawn(move || {
            for (x, y) in chunk {
                let ray = ray_for_pixel(&camera, x, y);
                let color = color_at(&world, ray);
                tx.send((x, y, color)).expect("Failed to send pixel data.");
            }
        });
    }

    drop(tx);

    let mut canvas = Canvas::new(camera.hsize, camera.vsize);
    let mut last_update = Instant::now();

    for (x, y, color) in rx {
        canvas.write_pixel(x, y, color);

        if last_update.elapsed() >= update_interval {
            on_update(&canvas);
            last_update = Instant::now();
        }
    }

    on_update(&canvas);

    canvas
}
