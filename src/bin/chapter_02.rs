extern crate rtc_rs as rtc;

use std::{fs::File, io::Write};

use rtc::{
    canvas::Canvas,
    color::Color,
    tuples::{point, vector, Tuple},
};

// Putting it together Chapter 2
#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Self {
        Self { position, velocity }
    }
}
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Environment {
    fn new(gravity: Tuple, wind: Tuple) -> Self {
        Self { gravity, wind }
    }
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let vel = proj.velocity + env.gravity + env.wind;
    let pos = proj.position + proj.velocity;

    return Projectile::new(pos, vel);
}

fn main() {
    let velocity = vector(1.0, 1.8, 0.0);

    let mut p = Projectile::new(point(0.0, 1.0, 0.0), velocity);
    let e = Environment::new(vector(0.0, -0.1, 0.0), vector(-0.01, 0.0, 0.0));

    let mut can = Canvas::new(300, 300);
    let mut values: Vec<(usize, usize)> = vec![];

    while p.position.y >= 0.0 {
                
        let x = can.width as f32 - p.position.x * can.width as f32;
        let y = can.height as f32 - p.position.y * can.height as f32;
        
        values.append(&mut vec![(x as usize, y as usize)]);
        println!(
            "Position values:\t{:?}\t{:?}",
            p.position.x, p.position.y
        );

        p = tick(&e, p);
    }

    for (x, y) in values {
        can.write_pixel(x, y, Color::white());
        println!("Position on canvas:\t{:?}\t\t{:?}", x, y);
    }

    let mut file = File::create("chapter_02.ppm").expect("Unable to create file.");

    file.write_all(&can.to_ppm().as_bytes())
        .expect("Unable to write file.");

    //print!("{}", can.to_ppm());
}
