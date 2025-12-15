pub struct Pixel {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

/// This function writes a single color in pixel's output
/// from [0, 1] space to [0, 256[
pub fn write_color(pixel_color: &Pixel) -> Pixel {
    let result = Pixel {
        r: pixel_color.r * 255.99,
        g: pixel_color.g * 255.99,
        b: pixel_color.b * 255.99,
    };

    result
}