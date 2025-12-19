use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// return a Vec3 coordinates of ray at distance t
    fn p(self, t: f64) -> Vec3 {
        Vec3::add(&self.origin,
        &Vec3::scalar_mult(&self.direction, &t))
    }
}