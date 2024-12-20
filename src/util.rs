
pub type RtcFl = f32;
const EPSILON: RtcFl = 0.0001;


pub fn equal(a: RtcFl, b: RtcFl) -> bool {
    // Compare two f32 values for equality within the constant EPSILON
    (a - b).abs() <= EPSILON
}