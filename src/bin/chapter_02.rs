extern crate rtc_rs as rtc;

use std::{fs::File, io::Write};

use rtc::{color::Color, canvas::Canvas, tuples::{point, vector, Tuple}};

// Putting it together Chapter 2
#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Self {
        Self {position, velocity}
    }
}
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Environment {
    fn new(gravity: Tuple, wind: Tuple) -> Self {
        Self {gravity, wind}
    }
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let pos = proj.position + proj.velocity;
    let vel = proj.velocity + env.gravity + env.wind;

    return Projectile::new(pos, vel);
}

fn main() {

    let velocity = vector(1.0, 1.8, 0.0) * 11.25;


    let mut p = Projectile::new(point(0.0, 1.0, 0.0), velocity.normalize());
    let e = Environment::new(vector(0.0, -0.1,0.0), vector(-0.01, 0.0, 0.0));

    let mut can = Canvas::new(900, 550);

    while p.position.y >= 0.0 {

        p = tick(&e, p);

        let p_n = p.position.normalize();

        let x = can.width as f32 - p_n.x * can.width as f32;
        let y = can.height as f32 - p_n.y * can.height as f32;
        can.write_pixel(x as usize, y as usize, Color::white());

        println!("Position on canvas: {:?} {:?}", x as usize, y as usize);
        
    }

    let mut file= File::create("chapter_01.ppm").expect("Unable to create file.");

        file.write_all(&can.to_ppm().as_bytes()).expect("Unable to write file.");

    //print!("{}", can.to_ppm());
}
