#[cfg(test)]
use crate::{canvas::Canvas, colors::Color};


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
    
    let ppm = c.to_ppm();

    assert_eq!(ppm, String::from("P3\n5 3\n255\n"));
}