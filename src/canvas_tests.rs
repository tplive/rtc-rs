use crate::{canvas::Canvas, colors::{color, Color}, util::RtcFl};


#[cfg(test)]

#[test]
fn creating_a_canvas() {
    use crate::{canvas::Canvas, util::RtcFl};


    let bits: usize = 3;
    let w = 10;
    let h = 20;
    let black: RtcFl = 0.0;

    let c = Canvas::new(w, h);

    let is_black = c.data.iter().all(|i: &RtcFl| i == &black);
    
    assert_eq!(c.width, w);
    assert_eq!(c.height, h);
    assert_eq!(c.data.len(), w as usize * h as usize * bits);
    
    assert_eq!(c.data.capacity(), w as usize * h as usize * bits);
    assert!(is_black);
}

#[test]
fn writing_pixels_to_a_canvas() {
    
    let w = 10;
    let h = 20;

    let c = Canvas::new(w, h);

    c.write_pixel(3, 3, color(0.2, 0.4, 0.6));

    assert!(c.pixel_at(3, 3).eq(&Color::new(0.2, 0.4, 0.6)));
    
}