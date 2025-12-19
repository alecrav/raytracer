pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Point3 type, for representing points in 3D space
/// Used only for avoiding confusion
pub type Point3 = Vec3;

// overloading operators
impl Vec3 {
    /// this functions adds two vectors
    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }


    /// this functions subtracts two vectors
    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    // this function computes the dot product between two vectors
    pub fn dot_product(&self, other: &Vec3) -> f64 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    /// this function compute the vector length
    pub fn vec_length(&self) -> f64 {
        (self.x.powi(2)
        + self.y.powi(2)
        + self.z.powi(2)).sqrt()
    }

    /// this function compute the vector normalization
    pub fn vec_normalize(&mut self) -> &mut Vec3 {
        self.x /= self.vec_length();
        self.y /= self.vec_length();
        self.z /= self.vec_length();

        self
    }

    /// this function computes the cross product between two vectors
    pub fn cross_product(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// scalar multiplication
    pub fn scalar_mult(&self, factor: &f64) -> Vec3 {
        Vec3 {  
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,  
        }
    }



    /// scalar division
    pub fn scalar_div(&mut self, factor: &f64) -> &mut Vec3 {
        self.x *= 1.0/factor;
        self.y *= 1.0/factor;
        self.z *= 1.0/factor;

        self
    }

    // TODO: other operations when I need them
    
}