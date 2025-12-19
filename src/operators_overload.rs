use std::ops;
use auto_ops::*;
use crate::vec3::Vec3;

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

impl_op_ex!(* |self: &Vec3, other &Vec3| -> f64 {
    self.x * other.x +
    self.y * other.y +
    self.z * other.z
});

impl_op_ex!(/ |vec: &Vec3, scale: f64| -> Vec3 {
    Vec3 {
        x: vec.x / scale,
        y: vec.y / scale,
        z: vec.z / scale,
    }
});