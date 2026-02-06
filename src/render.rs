use std::{
    sync::{mpsc, Arc},
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

const TILE_SIZE: usize = 16;

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

    let max_x = camera.hsize.saturating_sub(1);
    let max_y = camera.vsize.saturating_sub(1);

    let mut tiles: Vec<(usize, usize)> = Vec::new();
    let mut ty = 0;
    while ty < max_y {
        let mut tx0 = 0;
        while tx0 < max_x {
            tiles.push((tx0, ty));
            tx0 += TILE_SIZE;
        }
        ty += TILE_SIZE;
    }

    if tiles.is_empty() {
        return Canvas::new(camera.hsize, camera.vsize);
    }

    let tiles = Arc::new(tiles);
    let num_tiles = tiles.len();
    let chunk_size = num_tiles.div_ceil(num_threads);

    for start in (0..num_tiles).step_by(chunk_size) {
        let end = (start + chunk_size).min(num_tiles);
        let tx = tx.clone();
        let tiles = Arc::clone(&tiles);
        let world = world.clone();
        let camera = camera.clone();
        let bar = bar.clone();

        thread::spawn(move || {
            let tile_slice = &tiles[start..end];

            for (tx0, ty0) in tile_slice {
                let y_end = (*ty0 + TILE_SIZE).min(max_y);
                let x_end = (*tx0 + TILE_SIZE).min(max_x);

                for y in *ty0..y_end {
                    for x in *tx0..x_end {
                        bar.inc(1);
                        let ray = ray_for_pixel(&camera, x, y);
                        let color = color_at(&world, ray);
                        tx.send((x, y, color)).expect("Failed to send pixel data.");
                    }
                }
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

    let max_x = camera.hsize.saturating_sub(1);
    let max_y = camera.vsize.saturating_sub(1);

    let mut tiles: Vec<(usize, usize)> = Vec::new();
    let mut ty = 0;
    while ty < max_y {
        let mut tx0 = 0;
        while tx0 < max_x {
            tiles.push((tx0, ty));
            tx0 += TILE_SIZE;
        }
        ty += TILE_SIZE;
    }

    if tiles.is_empty() {
        return Canvas::new(camera.hsize, camera.vsize);
    }

    // Set to true to randomize pixels for effect
    // TODO: Implement GUI toggle
    if true {
        let mut rng = rand::rng();
        tiles.shuffle(&mut rng);
    }

    let tiles = Arc::new(tiles);
    let num_tiles = tiles.len();
    let chunk_size = num_tiles.div_ceil(num_threads);

    for start in (0..num_tiles).step_by(chunk_size) {
        let end = (start + chunk_size).min(num_tiles);
        let tx = tx.clone();
        let tiles = Arc::clone(&tiles);
        let world = world.clone();
        let camera = camera.clone();

        thread::spawn(move || {
            let tile_slice = &tiles[start..end];

            for (tx0, ty0) in tile_slice {
                let y_end = (*ty0 + TILE_SIZE).min(max_y);
                let x_end = (*tx0 + TILE_SIZE).min(max_x);

                for y in *ty0..y_end {
                    for x in *tx0..x_end {
                        let ray = ray_for_pixel(&camera, x, y);
                        let color = color_at(&world, ray);
                        tx.send((x, y, color)).expect("Failed to send pixel data.");
                    }
                }
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
