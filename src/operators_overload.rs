use crate::vec3::Vec3;
use auto_ops::*;

// in this implementation all operators are overloaded in
// such a way that  Vec3 + Vec3 is a supported operation
// for now only +, - and *. Using macros and using operation
// with both ref and owned value

impl_op_ex!(+ |vec: &Vec3, other: &Vec3| -> Vec3 {
    Vec3 {
        x: vec.x + other.x,
        y: vec.y + other.y,
        z: vec.z + other.z,
    }
});

impl_op_ex!(- |vec: &Vec3, other: &Vec3| -> Vec3 {
    Vec3 {
        x: vec.x - other.x,
        y: vec.y - other.y,
        z: vec.z - other.z,
    }
});

impl_op_ex!(*|vec: &Vec3, other: &Vec3| -> f64 {
    vec.x * other.x + vec.y * other.y + vec.z * other.z
});

impl_op_ex!(* |vec: &Vec3, scale: i32| -> Vec3 {
    Vec3 {
        x: vec.x * (scale as f64),
        y: vec.y * (scale as f64),
        z: vec.z * (scale as f64),
    }
});

impl_op_ex!(* |vec: &Vec3, scale: f64| -> Vec3 {
    Vec3 {
        x: vec.x * scale,
        y: vec.y * scale,
        z: vec.z * scale,
    }
});

impl_op_ex!(/ |vec: &Vec3, scale: f64| -> Vec3 {
    Vec3 {
        x: vec.x / scale,
        y: vec.y / scale,
        z: vec.z / scale,
    }
});
