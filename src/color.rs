use crate::{ray::{self, Ray}, vec3::Vec3};

pub type Pixel = Vec3;

/// This function writes a single color in pixel's output
/// from [0, 1] space to [0, 256[
pub fn write_color(pixel_color: &Pixel) -> Pixel {
    let result = Pixel {
        x: pixel_color.x * 255.99,
        y: pixel_color.y * 255.99,
        z: pixel_color.z * 255.99,
    };

    result
}

// return (1.0-a)*color(1.0, 1.0, 1.0) + a*color(0.5, 0.7, 1.0);
/// write function to have a gradient of a single color as output image
pub fn color_gradient(r: &Ray) -> Pixel {
    let unit_direction = r.direction.vec_normalize();
    let a = (unit_direction.y + 1.0) * 0.5;
    Vec3{ x:1.0, y: 1.0, z: 1.0 } * (1.0 - a)
    + Vec3 {x: 0.5, y: 0.7, z: 1.0} * a
}