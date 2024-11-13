use crate::rtc::colors::color;

#[test]
fn colors_are_rgb_tuples() {
    let c = color(-0.5, 0.4, 1.7);
    assert_eq!(c.red, -0.5);
    assert_eq!(c.green, 0.4);
    assert_eq!(c.blue, 1.7);
}