const EPSILON: f32 = 0.0001_f32;

pub type RtcFl = f32;

pub fn equal(a: f32, b: f32) -> bool {
    // Compare two f32 values for equality within the constant EPSILON
    (a - b).abs() <= EPSILON
}