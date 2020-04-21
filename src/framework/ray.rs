use crate::framework::vec3::Vec3;

/// A ray is a function:
/// p(t) = origin + direction*t
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Value of the ray function for a given t
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
