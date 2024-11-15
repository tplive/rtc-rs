//const EPSILON: f32 = 0.00001_f32;

pub type RtcFl = f32;

pub fn equal(a: f32, b: f32) -> bool {
    // Compare two f32 values for equality within the constant EPSILON
    (a - b).abs() <= f32::EPSILON // Try built-in EPSILON value, but change to my constant if it doesn't work out.
}