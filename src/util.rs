pub type RtcFl = f32;
pub const PI:RtcFl = std::f32::consts::PI;

pub const EPSILON: RtcFl = 0.0001;
pub const SHADOW_EPSILON: RtcFl = 0.01;

pub fn equal(a: RtcFl, b: RtcFl) -> bool {
    // Compare two f32 values for equality within the constant EPSILON
    (a - b).abs() <= EPSILON
}