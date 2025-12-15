use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    fn p(self, t: f64) -> Vec3 {
        Vec3::add(&self.origin,
        &Vec3::scalar_mult(&self.direction, &t))
    }
}