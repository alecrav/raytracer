use std::{num::TryFromIntError};
use crate::vec3::Vec3;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
    pub camera_direction: Vec3,
}

impl Camera {
    /// Get height function from width and aspect ratio, 
    /// making sure an i32 is returned
    pub fn get_image_height(&self) -> Result<i32, TryFromIntError> {
        let result_64 = self.image_width / self.aspect_ratio;

        match i32::try_from(result_64 as i64) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }
    
    /// Get viewport width from vp height, image width and aspect ratio
    pub fn get_viewport_width(&self) -> Result<i32, TryFromIntError> {
        let result_64 = self.viewport_height * (self.image_width / self.aspect_ratio);
        
        match i32::try_from(result_64 as i64) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }
}