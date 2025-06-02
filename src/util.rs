pub type RtcFl = f32;
pub const PI:RtcFl = std::f32::consts::PI;

pub const EPSILON: RtcFl = 0.0001;
pub const SHADOW_EPSILON: RtcFl = 0.01;

pub fn equal(a: RtcFl, b: RtcFl) -> bool {
    // Compare two f32 values for equality within the constant EPSILON
    (a - b).abs() <= EPSILON
}

/// Calculates image dimensions based on vertical size and optional aspect ratio.
/// 
/// If aspect != 0.0, hsize is calculated as vsize * aspect.
///
/// # Arguments
/// 
/// * `vsize` - Vertical size (height) in pixels
/// * `hsize` - Horisontal size (width) in pixels
/// 
/// # Returns
/// A tuple of `(vsize, hsize)` containing the calculated dimensions.
/// 
/// # Examples
/// ```
/// use rtc_rs::util::image_dimensions;
/// 
/// // Use provided dimensions
/// assert_eq!(image_dimensions(100, 200, 0.0), (100, 200));
/// 
/// // Calculate width from height and aspect ratio
/// assert_eq!(image_dimensions(1080, 100, 16.0/9.0), (1080, 1920));
/// 
/// // Square aspect ratio
/// assert_eq!(image_dimensions(2048, 55, 1.0), (2048, 2048));
/// 
/// // Landscape has aspect > 1.0
/// assert_eq!(image_dimensions(1024, 0, 1.3), (1024, 1331));
/// 
/// // Landscape has aspect > 1.0
/// assert_eq!(image_dimensions(1024, 0, 0.75), (1024, 768));
/// 
/// 
/// ```
/// A landscape format has aspect > 1.0. aspect < 1.0 for portrait.
/// 
/// # Panics
/// 
/// Panics if resulting dimensions are less than 1.
pub fn image_dimensions(vsize: usize, hsize: usize, aspect: f32) -> (usize, usize) {
    // If aspect is set to 0.0, use hsize. Else calculate hsize with width and aspect,
    // ignoring the input hsize.
    let mut hsize_tmp = hsize as f32;

    if aspect != 0.0 {
        hsize_tmp = vsize as f32 * aspect ;
    } 
    
    if vsize >= 1 && hsize_tmp.round() as usize >= 1 {
        (vsize, hsize_tmp as usize)
    } else {
        panic!("Image dimensions must be >= 1")
    }

}
