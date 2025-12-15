use std::{io, num::TryFromIntError};


pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: f64,
    pub viewport_height: f64,
}

impl Camera {
    /// Maybe overenginereed? get height function from width and aspect ratio, 
    /// making sure an i32 is returned
    pub fn get_image_heigth(&self) -> Result<i32, TryFromIntError> {
        let result_64 = self.image_width / self.aspect_ratio;

        match i32::try_from(result_64 as i64) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }

}