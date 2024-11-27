extern crate rtc_rs as rtc;

use std::{f32::consts::PI, fs::File, io::Write};

use rtc::{
    canvas::Canvas,
    color::Color,
    transformation::Transformation,
    tuples::point,
};

// Putting it together Chapter 4
fn main() {
    let width = 300.0;
    let height = 300.0;
    let radius = 6.0 / 8.0;

    let twelve = point(0.0, 0.0, 1.0 * radius);
    let mut can = Canvas::new(width as usize, height as usize);

    can.write_pixel((width/2.0) as usize, (height/2.0) as usize, Color::white());
    for n in 1..13 {
        let rotation = Transformation::new().rotation_y(n as f32 * PI / 6.0).get();
        let rotated = rotation * twelve;

        let x = width / 2.0 + (width / 2.0 * rotated.x);
        let y = height / 2.0 + (height / 2.0 * rotated.z);

        //println!("{:?}, {:?}", x.round(), y.round());
        can.write_rect(x as usize, y as usize, 4, 4, Color::new(0.543, 0.872, 0.32));
    }

    let mut file = File::create("chapter_04.ppm").expect("Unable to create file.");

    file.write_all(&can.to_ppm().as_bytes())
        .expect("Unable to write file.");
}
