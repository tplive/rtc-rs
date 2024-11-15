#[cfg(test)]
use crate::{canvas::Canvas, color::Color};


#[test]
fn creating_a_canvas() {
    use crate::canvas::Canvas;

    let w = 10;
    let h = 20;

    let c = Canvas::new(w, h);
    let color = Color::black();
    for x in 0..c.width -1 {
        for y in 0..c.height -1 {
            assert!(*c.pixel_at(x, y) == color);
        }
    }
    //let is_black = c.data.iter().all(|i: &RtcFl| i == &black);
    
    assert_eq!(c.width, w);
    assert_eq!(c.height, h);
    assert_eq!(c.data_size(), w as usize * h as usize);
    
}

#[test]
fn writing_pixels_to_a_canvas() {
    
    let w: usize = 10;
    let h: usize = 20;
    let color: Color = Color::new(0.2, 0.4, 0.6);
    let mut c: Canvas = Canvas::new(w, h);

    c.write_pixel(3, 3, color);

    let result: &Color = c.pixel_at(3, 3);

    assert!(result == &color);
    
}

#[test]
fn constructing_ppm_header() {
    let c = Canvas::new(5, 3);
    
    let ppm: String = c.to_ppm();
    let l1: String = ppm.lines().take(1).collect();
    let l2: String = ppm.lines().skip(1).take(1).collect();
    let l3: String = ppm.lines().skip(2).take(1).collect();

    assert!(l1 == "P3");
    assert!(l2 == format!("{} {}", c.width.to_string(), c.height.to_string()));
    assert!(l3 == "255");
}

#[test]
fn constructing_ppm_pixel_data() {
    let mut canvas = Canvas::new(5, 3);
    let color1 = Color::new(1.5, 0.0, 0.0);
    let color2 = Color::new(0.0, 0.5, 0.0);
    let color3 = Color::new(-0.5, 0.0, 1.0);

    canvas.write_pixel(0, 0, color1);
    canvas.write_pixel(2, 1, color2);
    canvas.write_pixel(4, 2, color3);

    let ppm = canvas.to_ppm();
    
    let expected_ppm = String::from("P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 255\n");

    // Debug data
    /*
    let exp_head = expected_ppm.lines().take(3);
    let exp_data = expected_ppm.lines().skip(3);
    
    println!("Header:");
    for n in exp_head {
        println!("{}", n);
    }

    println!("Data:");
    for n in exp_data {
        print!("{}\n", n);
    }

    println!();
    println!("ACTUAL PPM:\n{}", ppm);
    println!("EXPECTED PPM:\n{}", expected_ppm);
    */

    assert_eq!(ppm, expected_ppm);


}

#[test]
fn splitting_long_lines_in_ppm_files() {
    let mut c = Canvas::new(10, 2);
    for x in 0..10-1 {
        for y in 0..2-1 {
            c.write_pixel(x, y, Color::new(1.0, 0.8, 0.6));
        }
    }

    let ppm = c.to_ppm();
    let lines = ppm.lines().skip(3);

    for l in lines {
        assert!(l.len() <= 70);
    }
}

#[test]
fn ppm_ends_in_newline() {
    let c = Canvas::new(10, 2);
    let ppm = c.to_ppm();

    assert!(ppm.ends_with('\n'));

}