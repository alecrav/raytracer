use std::ops;
use crate::vec3::{Vec3, Point3};

/// in this implementation all operators are overloaded in
/// such a way that  Vec3 + Vec3 is a supported operation
/// for now only +, - and *

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// in this raytracer, dot product is represented
/// as vec1 * vec2
impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;
    
    fn mul(self, other: Vec3) -> f64 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }
}